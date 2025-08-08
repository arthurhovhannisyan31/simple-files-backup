use chrono::Local;
use regex::Regex;
use std::fs::{self, DirBuilder, File, copy, read_link, remove_dir_all, remove_file};
use std::io::Write;
use std::path::{MAIN_SEPARATOR_STR, Path, PathBuf};
use std::time::Instant;
use std::{env, io};

const LOG_FILE_NAME: &str = "files-backup-log.txt";

fn main() -> io::Result<()> {
  let start_time = Instant::now();
  let cur_path = env::current_dir().unwrap_or(PathBuf::from(MAIN_SEPARATOR_STR));
  let mut files_count: usize = 0;

  let home_path = Path::new("/home/q");
  let etc_path = Path::new("/etc");
  let backup_path = Path::new("/data/backup_2");

  let ignore_patterns: Vec<Regex> = vec![
    Regex::new(r"/node_modules/.+").unwrap(),
    Regex::new(r"/.yarn/.+").unwrap(),
    Regex::new(r"/.next/.+").unwrap(),
    Regex::new(r"/target/.+").unwrap(),
  ];

  let paths: Vec<(PathBuf, PathBuf)> = vec![
    (home_path.join("bin"), backup_path.join("bin")),
    (home_path.join(".config"), backup_path.join(".config")),
    (home_path.join(".ssh"), backup_path.join(".ssh")),
    (home_path.join(".bashrc"), backup_path.join(".bashrc")),
    (home_path.join(".gitconfig"), backup_path.join(".gitconfig")),
    (home_path.join(".Xmodmap"), backup_path.join(".Xmodmap")),
    (etc_path.join("fstab"), backup_path.join("fstab")),
    (etc_path.join("default/grub"), backup_path.join("grub")),
    (home_path.join("Documents"), backup_path.join("Documents")),
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

fn traverse_paths(
  paths: Vec<(PathBuf, PathBuf)>,
  ignore_patterns: Option<&Vec<Regex>>,
  files_count: &mut usize,
) -> io::Result<()> {
  for (source_path, target_path) in paths {
    if ignore_patterns.is_some() {
      for pattern in ignore_patterns.unwrap() {
        if pattern.is_match(source_path.to_str().unwrap()) {
          return Ok(());
        }
      }
    }

    let error_msg = format!("Cannot locate the path {:?}", &source_path);
    source_path.try_exists().expect(&error_msg);

    if source_path.is_dir() {
      if target_path.exists() {
        remove_dir_all(&target_path)?;
      }

      traverse_dir(
        &source_path,
        &source_path,
        &target_path,
        files_count,
        ignore_patterns,
      )?;
    } else if source_path.is_file() {
      backup_file(&source_path, &target_path)?;
      *files_count += 1;
    }
  }

  Ok(())
}

fn backup_file(source_path: &PathBuf, target_path: &PathBuf) -> io::Result<()> {
  if target_path.exists() {
    remove_file(target_path)?;
  }

  let create_new_error_msg = format!("Failed creating file {target_path:?}");
  File::create_new(target_path).expect(&create_new_error_msg);

  let copy_file_error_msg = format!("Failed copying file {target_path:?}");
  copy(source_path, target_path).expect(&copy_file_error_msg);

  Ok(())
}

fn traverse_dir(
  source_base_path: &PathBuf,
  source_path: &PathBuf,
  target_base_path: &PathBuf,
  files_count: &mut usize,
  ignore_patterns: Option<&Vec<Regex>>,
) -> io::Result<()> {
  let target_relative_path = source_path
    .strip_prefix(source_base_path)
    .expect("Failed getting relative path");

  let mut new_target_path = PathBuf::from(target_base_path);
  new_target_path.push(target_relative_path);

  if !new_target_path.exists() {
    DirBuilder::new().recursive(true).create(&new_target_path)?;
  }

  for entry in fs::read_dir(source_path)? {
    let entry = entry?;
    let entry_path = entry.path();

    if ignore_patterns.is_some() {
      for pattern in ignore_patterns.unwrap() {
        if pattern.is_match(entry_path.to_str().unwrap()) {
          return Ok(());
        }
      }
    }

    if entry_path.is_dir() {
      traverse_dir(
        source_base_path,
        &entry_path,
        target_base_path,
        files_count,
        ignore_patterns,
      )?;
    } else if let Some(file_name) = entry_path.file_name() {
      let mut target_path = new_target_path.clone();
      target_path.push(file_name);

      if entry_path.is_file() {
        backup_file(&entry_path, &target_path)?;
        *files_count += 1;
      } else {
        let link_path = read_link(&entry_path)?;
        let error_message = format!("Failed creating link for {target_path:?}");

        symlink(link_path, target_path).expect(&error_message);
        *files_count += 1;
      }
    }
  }

  Ok(())
}

#[cfg(windows)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
  std::os::windows::fs::symlink_file(original, link)
}

#[cfg(unix)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
  std::os::unix::fs::symlink(original, link)
}

fn write_logs(cur_path: &Path, log_str: &str) -> io::Result<()> {
  let mut file = File::options()
    .create(true)
    .append(true)
    .open(cur_path.join(LOG_FILE_NAME))
    .expect("Failed reading/creating log file");

  let mut new_content = String::from("----\n");
  new_content.push_str(format!("{}\n", Local::now().to_rfc2822()).as_str());
  new_content.push_str(log_str);

  file.write_all(new_content.as_bytes())?;
  file.sync_all()?;

  Ok(())
}
