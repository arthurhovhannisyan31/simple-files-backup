use std::fs::{File, copy, remove_file};
use std::io;
use std::path::{Path, PathBuf};

pub fn backup_file(
  source_path: &PathBuf,
  target_path: &PathBuf,
) -> io::Result<()> {
  if target_path.exists() {
    remove_file(target_path)?;
  }

  File::create_new(target_path)
    .unwrap_or_else(|_| panic!("Failed creating file {target_path:?}"));

  copy(source_path, target_path)
    .unwrap_or_else(|_| panic!("Failed copying file {target_path:?}"));

  Ok(())
}

#[cfg(windows)]
pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(
  original: P,
  link: Q,
) -> io::Result<()> {
  std::os::windows::fs::symlink_file(original, link)
}

#[cfg(unix)]
pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(
  original: P,
  link: Q,
) -> io::Result<()> {
  std::os::unix::fs::symlink(original, link)
}
