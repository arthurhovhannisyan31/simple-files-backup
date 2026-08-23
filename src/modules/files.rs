use std::fs::{copy, read_link, remove_file};
use std::io;
use std::path::{Path, PathBuf};

use crate::modules::constants::AppArror;

pub fn backup_file(
  source_path: &PathBuf,
  target_path: &PathBuf,
) -> Result<(), AppArror> {
  if target_path.exists() {
    remove_file(target_path).map_err(|err| AppArror::RemoveFileError {
      source_path: target_path.to_string_lossy().into_owned(),
      err,
    })?;
  }

  copy(source_path, target_path).map_err(|err| AppArror::CopyFileError {
    source_path: source_path.to_string_lossy().into_owned(),
    target_path: target_path.to_string_lossy().into_owned(),
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
) -> Result<(), AppArror> {
  if target_path.exists() {
    remove_file(target_path).map_err(|err| AppArror::RemoveFileError {
      source_path: target_path.to_string_lossy().into_owned(),
      err,
    })?
  }

  let link_path =
    read_link(source_path).map_err(|err| AppArror::ReadFileError {
      source_path: source_path.to_string_lossy().into_owned(),
      err,
    })?;

  symlink(link_path, target_path).map_err(|err| {
    AppArror::CreateSymlinkError {
      source_path: source_path.to_string_lossy().into_owned(),
      target_path: target_path.to_string_lossy().into_owned(),
      err,
    }
  })?;

  Ok(())
}
