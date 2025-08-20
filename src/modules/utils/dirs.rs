use std::fs::{self, DirBuilder, remove_dir_all};
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;

use regex::Regex;

use crate::modules::types::BackupCommand;

pub fn traverse_files(
  command_sender: mpsc::Sender<BackupCommand>,
  source: Vec<PathBuf>,
  target: PathBuf,
  ignore: Option<&Regex>,
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
    let file_target_path = target.join(file_name);

    if source_path.is_dir() {
      if file_target_path.exists() {
        remove_dir_all(&file_target_path)?;
      }

      traverse_dir(
        command_sender.clone(),
        &source_path,
        &source_path,
        &file_target_path,
        ignore,
      )?;
    } else {
      command_sender
        .send((source_path, file_target_path))
        .expect("Failed sending backup command");
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
) -> io::Result<()> {
  let target_relative_path = source_path
    .strip_prefix(source_base_path)
    .expect("Failed getting source file relative path");

  let mut new_target_path = PathBuf::from(target_base_path);
  new_target_path.push(target_relative_path);

  if !new_target_path.exists() {
    DirBuilder::new().recursive(true).create(&new_target_path)?;
  }

  for entry in fs::read_dir(source_path)? {
    let entry = entry?;
    let entry_path = entry.path();

    if ignore.is_some()
      && ignore.unwrap().is_match(source_path.to_str().unwrap())
    {
      return Ok(());
    }

    if entry_path.is_dir() {
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
        .expect("Failed sending backup command");
    }
  }

  Ok(())
}
