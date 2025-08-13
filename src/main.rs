mod modules;

use regex::Regex;
use std::path::{MAIN_SEPARATOR_STR, Path, PathBuf};
use std::time::Instant;
use std::{env, io};

const LOG_FILE_NAME: &str = "files-backup-log.txt";

use modules::helpers::{traverse_paths, write_logs};

fn main() -> io::Result<()> {
  let start_time = Instant::now();
  let cur_path = env::current_dir().unwrap_or(PathBuf::from(MAIN_SEPARATOR_STR));
  let mut files_count: usize = 0;

  let home_path = Path::new("/home/q");
  let etc_path = Path::new("/etc");
  let backup_path = Path::new("/data/backup_2");

  let ignore_patterns = Regex::new(r"/(node_modules|.yarn|.next|target|yarn.lock)")
    .unwrap_or_else(|_| panic!("Failed parsing regex"));

  let paths: Vec<(PathBuf, PathBuf)> = vec![
    (home_path.join("bin"), backup_path.join("bin")),
    (home_path.join(".config"), backup_path.join(".config")),
    (home_path.join(".ssh"), backup_path.join(".ssh")),
    (home_path.join(".bashrc"), backup_path.join(".bashrc")),
    // (home_path.join(".gitconfig"), backup_path.join(".gitconfig")),
    // (home_path.join(".Xmodmap"), backup_path.join(".Xmodmap")),
    // (etc_path.join("fstab"), backup_path.join("fstab")),
    // (etc_path.join("default/grub"), backup_path.join("grub")),
    // (home_path.join("Documents"), backup_path.join("Documents")),
  ];

  let log_message = match traverse_paths(paths, Some(&ignore_patterns), &mut files_count) {
    Ok(_) => format!(
      "Backup complete in {:?}, {} files were backed up\n",
      start_time.elapsed(),
      files_count
    ),
    Err(err) => format!("{err:?}"),
  };

  write_logs(&cur_path, &log_message)?;

  Ok(())
}
