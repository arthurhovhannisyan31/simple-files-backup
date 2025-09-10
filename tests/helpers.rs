use std::path::PathBuf;
use std::thread;

use assert_fs::TempDir;
use assert_fs::fixture::{FileTouch, FileWriteStr, PathChild};

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

pub fn setup_config_file(
  root_dir: &TempDir,
  name: &str,
  source: Option<PathBuf>,
  target: Option<PathBuf>,
  ignore: Option<&str>,
  log: Option<PathBuf>,
) {
  let config_file = root_dir.child(name);
  let mut content_strings: Vec<String> = vec![];
  let mut config_file_content = String::new();

  config_file_content.push_str("{\n");

  if let Some(target) = target {
    content_strings.push(format!("\t\"target\": {:?}", target));
  }
  if let Some(source) = source {
    content_strings.push(format!("\t\"source\": [{:?}]", source));
  }
  if let Some(ignore) = ignore {
    content_strings.push(format!("\t\"ignore\": {:?}", ignore));
  }
  if let Some(log) = log {
    content_strings.push(format!("\t\"log\": {:?}", log));
  }

  config_file_content.push_str(content_strings.join(",\n").as_str());
  config_file_content.push_str("\n}");

  config_file.write_str(config_file_content.as_str()).unwrap();
}

pub fn setup_files(source_dir: &TempDir, file_paths: &Vec<&str>) {
  for file_path in file_paths {
    source_dir.child(file_path).touch().unwrap();
  }
}
