use std::thread;

use assert_fs::TempDir;

#[cfg(unix)]
fn get_temp_dir() -> TempDir {
  TempDir::new().unwrap()
}
#[cfg(windows)]
fn get_temp_dir() -> TempDir {
  TempDir::new_in(".").unwrap()
}

pub fn setup_dirs() -> (TempDir, TempDir, TempDir) {
  let thread_id = thread::current().id();
  let root_dir = get_temp_dir();
  let source_dir = TempDir::with_prefix_in(
    format!("source-{:?}-", thread_id),
    root_dir.path(),
  )
  .unwrap();
  let target_dir = TempDir::with_prefix_in(
    format!("target-{:?}-", thread_id),
    root_dir.path(),
  )
  .unwrap();

  (root_dir, source_dir, target_dir)
}
