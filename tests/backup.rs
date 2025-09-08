mod helpers;

#[cfg(test)]
mod test {
  use assert_fs::prelude::*;
  use predicates::Predicate;
  use regex::Regex;

  use crate::helpers::setup_dirs;
  use simple_files_backup::modules::backup::backup;

  const THREADS_COUNT: usize = 2;

  #[test]
  fn test_backup_files() {
    let (root_dir, source_dir, target_dir) = setup_dirs();
    let mut files_count: usize = 0;
    let file_paths = vec![
      "test.txt",
      "subdir/test.txt",
      "subdir/subdir/test.txt",
      "subdir/subdir/subdir/test.txt",
      "subdir/subdir/subdir/subdir/test.txt",
      "subdir/subdir/subdir/subdir/subdir/test.txt",
      "subdir/subdir/subdir/subdir/subdir/subdir/test.txt",
      "subdir/subdir/subdir/subdir/subdir/subdir/subdir/test.txt",
    ];

    for file_path in &file_paths {
      source_dir.child(file_path).touch().unwrap();
    }

    let source_paths = vec![source_dir.to_path_buf()];
    let target_path = target_dir.to_path_buf();

    backup(
      source_paths,
      target_path,
      None,
      &mut files_count,
      THREADS_COUNT,
    );

    for file_path in &file_paths {
      assert!(
        predicates::path::exists()
          .eval(&source_dir.join(file_path).to_path_buf())
      );
    }
    assert_eq!(files_count, file_paths.len());
    root_dir.close().unwrap();
  }

  #[test]
  fn test_backup_ignore_pattern() {
    let (root_dir, source_dir, target_dir) = setup_dirs();
    let mut files_count: usize = 0;
    let file_paths = vec![
      "yarn.lock",
      "subdir/yarn.lock",
      "node_modules",
      "subdir/node_modules",
      ".yarn",
      "subdir/.yarn",
      ".next",
      "subdir/.next",
      "target",
      "subdir/target",
      "test.txt",
      "subdir/test.txt",
    ];
    let ignore_pattern =
      Some(Regex::new(r"(node_modules|.yarn|.next|target|yarn.lock)").unwrap());

    for file_path in &file_paths {
      source_dir.child(file_path).touch().unwrap();
    }

    for file_path in &file_paths {
      assert!(
        predicates::path::exists()
          .eval(&source_dir.join(file_path).to_path_buf())
      );
    }

    let source_paths = vec![source_dir.to_path_buf()];
    let target_path = target_dir.to_path_buf();

    backup(
      source_paths,
      target_path,
      ignore_pattern,
      &mut files_count,
      THREADS_COUNT,
    );

    let target_folder_path =
      target_dir.join(source_dir.components().next_back().unwrap());

    assert!(
      predicates::path::exists().eval(&target_folder_path.join("test.txt"))
    );
    assert!(
      predicates::path::exists()
        .eval(&target_folder_path.join("subdir").join("test.txt"))
    );
    assert_eq!(files_count, 2);
    root_dir.close().unwrap();
  }

  #[test]
  fn test_backup_contents() {
    let (root_dir, source_dir, target_dir) = setup_dirs();
    let mut files_count: usize = 0;
    let file_name = "test.txt";
    let file_content = "file content";

    let source_file = source_dir.child(file_name);
    source_file.write_str(file_content).unwrap();

    backup(
      vec![source_dir.to_path_buf()],
      target_dir.to_path_buf(),
      None,
      &mut files_count,
      THREADS_COUNT,
    );

    let target_path = target_dir
      .join(source_dir.components().next_back().unwrap())
      .join(file_name);

    assert!(
      predicates::path::eq_file(source_file.path()).eval(target_path.as_path())
    );
    root_dir.close().unwrap();
  }

  #[test]
  fn test_backup_file_symlink() {
    let (root_dir, source_dir, target_dir) = setup_dirs();
    let mut files_count: usize = 0;
    let file_name = "test.txt";
    let link_file_name = "link";

    let source_file = source_dir.child(file_name);
    source_file.touch().unwrap();

    source_dir
      .child(link_file_name)
      .symlink_to_file(&source_file)
      .unwrap();

    backup(
      vec![source_dir.to_path_buf()],
      target_dir.to_path_buf(),
      None,
      &mut files_count,
      THREADS_COUNT,
    );

    let target_link_path = target_dir
      .join(source_dir.components().next_back().unwrap())
      .join(link_file_name);

    assert!(predicates::path::exists().eval(&target_link_path));
    assert!(predicates::path::is_symlink().eval(&target_link_path));
    root_dir.close().unwrap();
  }

  #[cfg(not(windows))]
  #[test]
  fn test_backup_dir_symlink() {
    let (root_dir, source_dir, target_dir) = setup_dirs();
    let mut files_count: usize = 0;
    let child_dir_name = "subdir";
    let file_name = "test.txt";
    let link_file_name = "link";

    let child_dir = source_dir.join(child_dir_name);

    source_dir
      .child(child_dir_name)
      .child(file_name)
      .touch()
      .unwrap();

    source_dir
      .child(link_file_name)
      .symlink_to_dir(&child_dir)
      .unwrap();

    backup(
      vec![source_dir.to_path_buf()],
      target_dir.to_path_buf(),
      None,
      &mut files_count,
      THREADS_COUNT,
    );

    let target_link_path = target_dir
      .join(source_dir.components().next_back().unwrap())
      .join(link_file_name);

    // TODO Fix, fails on windows
    assert!(predicates::path::exists().eval(&target_link_path));
    assert!(predicates::path::is_symlink().eval(&target_link_path));
    root_dir.close().unwrap();
  }
}
