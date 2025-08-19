use regex::Regex;
use std::fs::{self, DirBuilder, read_link, remove_dir_all};
use std::io;
use std::path::PathBuf;

use crate::modules::utils::files::{backup_file, backup_symlink, symlink};

pub fn traverse_paths(
  source: Vec<PathBuf>,
  ignore: Option<&Regex>,
  target_path: PathBuf,
  files_count: &mut usize,
) -> io::Result<()> {
  for source_path in source {
    if ignore.is_some()
      && ignore.unwrap().is_match(source_path.to_str().unwrap())
    {
      return Ok(());
    }

    let copy_source_path = &source_path;
    source_path.try_exists().unwrap_or_else(|_| {
      panic!("Cannot locate the path {copy_source_path:?}")
    });

    let file_name = source_path
      .file_name()
      .expect("Failed reading file/dir name");
    let file_target_path = target_path.join(file_name);

    if source_path.is_dir() {
      if file_target_path.exists() {
        remove_dir_all(&file_target_path)?;
      }

      traverse_dir(
        &source_path,
        &source_path,
        &file_target_path,
        files_count,
        ignore,
      )?;
    } else {
      if source_path.is_file() {
        backup_file(&source_path, &file_target_path)?;
      } else if source_path.is_symlink() {
        backup_symlink(&source_path, &target_path)?;
      }

      *files_count += 1;
    }
  }

  Ok(())
}

fn traverse_dir(
  source_base_path: &PathBuf,
  source_path: &PathBuf,
  target_base_path: &PathBuf,
  files_count: &mut usize,
  ignore_pattern: Option<&Regex>,
) -> io::Result<()> {
  let target_relative_path = source_path
    .strip_prefix(source_base_path)
    .expect("Failed getting relative path");

  let mut new_target_path = PathBuf::from(target_base_path);
  new_target_path.push(target_relative_path);

  if !new_target_path.exists() {
    DirBuilder::new().recursive(true).create(&new_target_path)?;
  }

  for entry in fs::read_dir(source_path)? {
    let entry = entry?;
    let entry_path = entry.path();

    if ignore_pattern.is_some()
      && ignore_pattern
        .unwrap()
        .is_match(source_path.to_str().unwrap())
    {
      return Ok(());
    }

    if entry_path.is_dir() {
      traverse_dir(
        source_base_path,
        &entry_path,
        target_base_path,
        files_count,
        ignore_pattern,
      )?;
    } else if let Some(file_name) = entry_path.file_name() {
      let mut target_path = new_target_path.clone();
      target_path.push(file_name);

      if entry_path.is_file() {
        backup_file(&entry_path, &target_path)?;
      } else if entry_path.is_symlink() {
        backup_symlink(&entry_path, &target_path)?;
      }

      *files_count += 1;
    }
  }

  Ok(())
}
