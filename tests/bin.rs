mod helpers;
mod stubs;

#[cfg(test)]
mod test_bin {
  use std::env;

  use assert_cmd::Command;

  use crate::helpers::{setup_config_file, setup_dirs, setup_files};
  use crate::stubs::get_file_path_stubs;

  #[cfg(unix)]
  #[test]
  fn test_valid_setup() {
    let cur_dir = env::current_dir().unwrap();
    let (root_dir, source_dir, target_dir) = setup_dirs();
    let file_paths: Vec<&str> = get_file_path_stubs();
    let bin_path = cur_dir.join("bin/simple-files-backup");
    let config_file_path = root_dir.join("config.json");
    let log_file_path = root_dir.join("log.txt");

    let mut cmd = Command::cargo_bin(bin_path.to_str().unwrap()).unwrap();

    setup_files(&source_dir, &file_paths);
    setup_config_file(
      &root_dir,
      "config.json",
      Some(source_dir.to_path_buf()),
      Some(target_dir.to_path_buf()),
      Some("(node_modules|.yarn|.next|target|yarn.lock)"),
      Some(log_file_path),
    );

    cmd.arg("--config").arg(config_file_path).assert().success();

    root_dir.close().unwrap();
  }

  #[cfg(unix)]
  #[test]
  fn test_missing_config_file() {
    let cur_dir = env::current_dir().unwrap();
    let (root_dir, source_dir, _target_dir) = setup_dirs();
    let file_paths: Vec<&str> = get_file_path_stubs();
    let bin_path = cur_dir.join("bin/simple-files-backup");
    let config_file_path = root_dir.join("config.json");

    let mut cmd = Command::cargo_bin(bin_path.to_str().unwrap()).unwrap();

    setup_files(&source_dir, &file_paths);

    cmd.arg("--config").arg(config_file_path).assert().failure();

    root_dir.close().unwrap();
  }
  // TODO Write os specific tests
}
