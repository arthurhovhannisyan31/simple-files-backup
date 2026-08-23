use std::fs::{self, DirBuilder, remove_dir_all};
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::sync::mpsc;

use anyhow::anyhow;
use regex::Regex;

use crate::modules::constants::AppArror;
use crate::modules::types::BackupCommand;

/// Traverses every source path, streaming per-file backup commands to the
/// worker pool. Directory sources are staged into a temporary sibling
/// directory so the previous backup is never destroyed before the new copy
/// completes. Returns the list of `(staging_path, final_path)` pairs the
/// caller must swap into place once all copies have finished.
pub fn traverse_sources(
  command_sender: mpsc::Sender<BackupCommand>,
  source: Vec<PathBuf>,
  target: PathBuf,
  ignore: Option<&Regex>,
) -> Result<Vec<(PathBuf, PathBuf)>, AppArror> {
  let mut pending_swaps: Vec<(PathBuf, PathBuf)> = Vec::new();

  for entry_path in source {
    if let Some(ignore) = ignore {
      if ignore.is_match(&entry_path.to_string_lossy()) {
        continue;
      }
    }

    let exists = entry_path.try_exists().map_err(|err| AppArror::NotFound {
      source_path: entry_path.to_string_lossy().into_owned(),
      err,
    })?;
    if !exists {
      return Err(AppArror::NotFound {
        source_path: entry_path.to_string_lossy().into_owned(),
        err: io::Error::new(ErrorKind::NotFound, "Source path does not exist"),
      });
    }

    let file_name =
      &entry_path
        .file_name()
        .ok_or_else(|| AppArror::ReadFileError {
          source_path: entry_path.to_string_lossy().into_owned(),
          err: io::Error::new(
            ErrorKind::InvalidFilename,
            "Failed reading file/dir name",
          ),
        })?;
    let file_target_path = target.join(file_name);

    let meta = fs::symlink_metadata(&entry_path).map_err(|err| {
      AppArror::ReadFileError {
        source_path: entry_path.to_string_lossy().into_owned(),
        err,
      }
    })?;

    if meta.is_dir() {
      // Stage into a temporary sibling directory instead of deleting the
      // existing backup up front. The swap into `file_target_path` happens
      // only after every copy has completed.
      let mut staging_name = file_name.to_os_string();
      staging_name.push(".backup");
      let staging_path = target.join(&staging_name);

      if staging_path.exists() {
        remove_dir_all(&staging_path).map_err(|err| {
          AppArror::RemoveFileError {
            source_path: staging_path.to_string_lossy().into_owned(),
            err,
          }
        })?;
      }

      traverse_dir(
        command_sender.clone(),
        &entry_path,
        &entry_path,
        &staging_path,
        ignore,
      )?;

      pending_swaps.push((staging_path, file_target_path));
    } else {
      command_sender
        .send((entry_path, file_target_path))
        .map_err(|err| {
          AppArror::OtherError(anyhow!(
            "Failed sending backup command: {}",
            err
          ))
        })?;
    }
  }

  Ok(pending_swaps)
}

fn traverse_dir(
  command_sender: mpsc::Sender<BackupCommand>,
  source_base_path: &PathBuf,
  source_path: &PathBuf,
  target_base_path: &PathBuf,
  ignore: Option<&Regex>,
) -> Result<(), AppArror> {
  let target_relative_path = source_path
    .strip_prefix(source_base_path)
    .map_err(|err| AppArror::ReadFileError {
      source_path: source_path.to_string_lossy().into_owned(),
      err: io::Error::new(
        ErrorKind::InvalidFilename,
        format!("Failed stripping path prefix: {:?}", err),
      ),
    })?;

  let mut new_target_path = PathBuf::from(target_base_path);
  new_target_path.push(target_relative_path);

  if !new_target_path.exists() {
    DirBuilder::new()
      .recursive(true)
      .create(&new_target_path)
      .map_err(|err| AppArror::CreateFileError {
        target_path: new_target_path.to_string_lossy().into_owned(),
        err,
      })?;
  }

  let entries =
    fs::read_dir(source_path).map_err(|err| AppArror::ReadDirError {
      source_path: source_path.to_string_lossy().into_owned(),
      err,
    })?;

  for entry in entries {
    let entry = entry.map_err(|err| AppArror::ReadFileError {
      source_path: source_path.to_string_lossy().into_owned(),
      err,
    })?;
    let entry_path = entry.path();

    if let Some(ignore) = ignore
      && ignore.is_match(&entry_path.to_string_lossy())
    {
      continue;
    }

    let meta = fs::symlink_metadata(&entry_path).map_err(|err| {
      AppArror::ReadFileError {
        source_path: entry_path.to_string_lossy().into_owned(),
        err,
      }
    })?;
    {
      if meta.is_dir() {
        traverse_dir(
          command_sender.clone(),
          source_base_path,
          &entry_path,
          target_base_path,
          ignore,
        )?;
      } else if let Some(file_name) = entry_path.file_name() {
        let mut target_path = new_target_path.clone();
        target_path.push(file_name);

        command_sender
          .send((entry_path, target_path))
          .map_err(|err| {
            AppArror::OtherError(anyhow!(
              "Failed sending backup command: {}",
              err
            ))
          })?;
      }
    }
  }

  Ok(())
}
