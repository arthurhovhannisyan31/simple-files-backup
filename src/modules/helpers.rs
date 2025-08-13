use crate::LOG_FILE_NAME;
use chrono::Local;
use regex::Regex;
use std::fs::{DirBuilder, File, copy, read_link, remove_dir_all, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn traverse_paths(
  paths: Vec<(PathBuf, PathBuf)>,
  ignore_pattern: Option<&Regex>,
  files_count: &mut usize,
) -> io::Result<()> {
  for (source_path, target_path) in paths {
    if ignore_pattern.is_some()
      && ignore_pattern
        .unwrap()
        .is_match(source_path.to_str().unwrap())
    {
      return Ok(());
    }

    let copy_source_path = &source_path;
    source_path
      .try_exists()
      .unwrap_or_else(|_| panic!("Cannot locate the path {copy_source_path:?}"));

    if source_path.is_dir() {
      if target_path.exists() {
        remove_dir_all(&target_path)?;
      }

      traverse_dir(
        &source_path,
        &source_path,
        &target_path,
        files_count,
        ignore_pattern,
      )?;
    } else if source_path.is_file() {
      backup_file(&source_path, &target_path)?;
      *files_count += 1;
    }
  }

  Ok(())
}

fn backup_file(source_path: &PathBuf, target_path: &PathBuf) -> io::Result<()> {
  if target_path.exists() {
    remove_file(target_path)?;
  }

  File::create_new(target_path).unwrap_or_else(|_| panic!("Failed creating file {target_path:?}"));

  copy(source_path, target_path).unwrap_or_else(|_| panic!("Failed copying file {target_path:?}"));

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
        *files_count += 1;
      } else {
        let link_path = read_link(&entry_path)?;
        symlink(&link_path, &target_path)
          .unwrap_or_else(|_| panic!("Failed creating link for {target_path:?}"));
        *files_count += 1;
      }
    }
  }

  Ok(())
}

#[cfg(windows)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
  std::os::windows::fs::symlink_file(original, link)
}

#[cfg(unix)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
  std::os::unix::fs::symlink(original, link)
}

pub fn write_logs(cur_path: &Path, log_str: &str) -> io::Result<()> {
  let mut file = File::options()
    .create(true)
    .append(true)
    .open(cur_path.join(LOG_FILE_NAME))
    .expect("Failed reading/creating log file");

  let mut new_content = String::from("----\n");
  new_content.push_str(format!("{}\n", Local::now().to_rfc2822()).as_str());
  new_content.push_str(log_str);

  file.write_all(new_content.as_bytes())?;
  file.sync_all()?;

  Ok(())
}
