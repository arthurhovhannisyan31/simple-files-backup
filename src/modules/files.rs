use std::fs::{File, copy, read_link, remove_file};
use std::io;
use std::path::{Path, PathBuf};

use crate::modules::constants::FSErrors;

pub fn backup_file(
  source_path: &PathBuf,
  target_path: &PathBuf,
) -> Result<(), FSErrors> {
  if target_path.exists() {
    remove_file(target_path).map_err(|err| FSErrors::RemoveFileError {
      source_path: String::from(target_path.to_str().unwrap()),
      err,
    })?;
  }
  File::create_new(target_path).map_err(|err| FSErrors::CreateFileError {
    target_path: String::from(source_path.to_str().unwrap()),
    err,
  })?;

  copy(source_path, target_path).map_err(|err| FSErrors::CopyFileError {
    source_path: String::from(source_path.to_str().unwrap()),
    target_path: String::from(target_path.to_str().unwrap()),
    err,
  })?;

  Ok(())
}

#[cfg(windows)]
pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(
  original: P,
  link: Q,
) -> io::Result<()> {
  std::os::windows::fs::symlink_file(original, link)
}

#[cfg(not(windows))]
pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(
  original: P,
  link: Q,
) -> io::Result<()> {
  std::os::unix::fs::symlink(original, link)
}

pub fn backup_symlink(
  source_path: &PathBuf,
  target_path: &PathBuf,
) -> Result<(), FSErrors> {
  if target_path.exists() {
    remove_file(target_path).map_err(|err| FSErrors::RemoveFileError {
      source_path: String::from(target_path.to_str().unwrap()),
      err,
    })?
  }

  let link_path =
    read_link(source_path).map_err(|err| FSErrors::ReadFileError {
      source_path: String::from(target_path.to_str().unwrap()),
      err,
    })?;

  symlink(link_path, target_path).map_err(|err| {
    FSErrors::CreateSymlinkError {
      source_path: String::from(source_path.to_str().unwrap()),
      target_path: String::from(target_path.to_str().unwrap()),
      err,
    }
  })?;

  Ok(())
}
