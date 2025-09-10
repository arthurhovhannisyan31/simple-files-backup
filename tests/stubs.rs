pub fn get_file_path_stubs() -> Vec<&'static str> {
  vec![
    "test.txt",
    "subdir/test.txt",
    "subdir/subdir/test.txt",
    "subdir/subdir/subdir/test.txt",
    "subdir/subdir/subdir/subdir/test.txt",
    "subdir/subdir/subdir/subdir/subdir/test.txt",
    "subdir/subdir/subdir/subdir/subdir/subdir/test.txt",
    "subdir/subdir/subdir/subdir/subdir/subdir/subdir/test.txt",
  ]
}

pub fn get_ignored_file_path_stubs() -> Vec<&'static str> {
  vec![
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
  ]
}
