use std::fs::{self, DirBuilder, remove_dir_all};
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::sync::mpsc;

use anyhow::anyhow;
use regex::Regex;

use crate::modules::constants::FSErrors;
use crate::modules::types::BackupCommand;

pub fn traverse_sources(
  command_sender: mpsc::Sender<BackupCommand>,
  source: Vec<PathBuf>,
  target: PathBuf,
  ignore: Option<&Regex>,
) -> Result<(), FSErrors> {
  for entry_path in source {
    if ignore.is_some()
      && ignore.unwrap().is_match(entry_path.to_str().unwrap())
    {
      return Ok(());
    }

    entry_path.try_exists().map_err(|err| FSErrors::NotFound {
      source_path: String::from(entry_path.to_str().unwrap()),
      err,
    })?;

    let file_name =
      &entry_path
        .file_name()
        .ok_or_else(|| FSErrors::ReadFileError {
          source_path: String::from(entry_path.to_str().unwrap()),
          err: io::Error::new(
            ErrorKind::InvalidFilename,
            "Failed reading file/dir name",
          ),
        })?;
    let file_target_path = target.join(file_name);

    if let Ok(meta) = fs::symlink_metadata(&entry_path) {
      if meta.is_dir() {
        if file_target_path.exists() {
          remove_dir_all(&file_target_path).map_err(|err| {
            FSErrors::RemoveFileError {
              source_path: String::from(file_target_path.to_str().unwrap()),
              err,
            }
          })?;
        }

        traverse_dir(
          command_sender.clone(),
          &entry_path,
          &entry_path,
          &file_target_path,
          ignore,
        )?;
      } else {
        command_sender
          .send((entry_path, file_target_path))
          .map_err(|err| {
            FSErrors::OtherError(anyhow!(
              "Failed sending backup command: {}",
              err
            ))
          })?;
      }
    }
  }

  Ok(())
}

fn traverse_dir(
  command_sender: mpsc::Sender<BackupCommand>,
  source_base_path: &PathBuf,
  source_path: &PathBuf,
  target_base_path: &PathBuf,
  ignore: Option<&Regex>,
) -> Result<(), FSErrors> {
  let target_relative_path = source_path
    .strip_prefix(source_base_path)
    .map_err(|err| FSErrors::ReadFileError {
      source_path: String::from(source_path.to_str().unwrap()),
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
      .map_err(|err| FSErrors::CreateFileError {
        target_path: String::from(new_target_path.to_str().unwrap()),
        err,
      })?;
  }

  let entries =
    fs::read_dir(source_path).map_err(|err| FSErrors::ReadDirError {
      source_path: String::from(source_path.to_str().unwrap()),
      err,
    })?;

  for entry in entries {
    let entry = entry.map_err(|err| FSErrors::ReadFileError {
      source_path: String::from(source_path.to_str().unwrap()),
      err,
    })?;
    let entry_path = entry.path();

    if ignore.is_some()
      && ignore.unwrap().is_match(entry_path.to_str().unwrap())
    {
      continue;
    }

    if let Ok(meta) = fs::symlink_metadata(&entry_path) {
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
            FSErrors::OtherError(anyhow!(
              "Failed sending backup command: {}",
              err
            ))
          })?;
      }
    }
  }

  Ok(())
}
