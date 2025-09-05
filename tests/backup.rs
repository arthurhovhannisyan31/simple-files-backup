mod helpers;

#[cfg(test)]
mod test {
  use std::path::PathBuf;

  use assert_fs::prelude::*;
  use predicates::Predicate;

  use crate::helpers::setup_dirs;
  use simple_files_backup::modules::backup::backup;

  #[test]
  fn test_dir() {}

  #[test]
  fn test_file() {}

  #[test]
  fn test_symlink() {}

  #[test]
  fn test_nested_files() {}

  #[test]
  fn test_backup() {
    let (_root_dir, source_dir, target_dir) = setup_dirs();

    // Setup files - setup as a helper
    source_dir.child("test.txt").touch().unwrap();
    source_dir.child("subdir/test.txt").touch().unwrap();

    // Setup backup configs
    let source_paths = vec![source_dir.to_path_buf()];
    let target_path = target_dir.to_path_buf();
    let mut files_count: usize = 0;
    let threads_count = 2;

    backup(
      source_paths,
      target_path,
      None,
      &mut files_count,
      threads_count,
    );

    let paths: Vec<PathBuf> = vec![
      source_dir.join("test.txt").to_path_buf(),
      source_dir.join("subdir").to_path_buf(),
      source_dir.join("subdir/test.txt").to_path_buf(),
    ];

    paths.iter().for_each(|path_buf| {
      assert!(predicates::path::exists().eval(path_buf));
    });
  }
}

// cases:
// single file, content equal
// single file and symlink to file, symlink has same reference
// file in directory
// file in directory and symlink to file
// file in directory and symlink to directory
// existing source files, existing target dir
// missing source files, existing target dir
// missing source files, missing target dir
// existing symlink to dir, existing target dir
// existing symlink to file, existing target dir
// prove that copied file content is the same
// prove that copied symlink has same reference as original
// test with and without ignore pattern
