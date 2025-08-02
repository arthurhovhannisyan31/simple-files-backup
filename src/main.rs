use std::fs::{self, DirBuilder, DirEntry, File};
use std::io::Write;
use std::path::{MAIN_SEPARATOR_STR, Path, PathBuf};
use std::{env, io};

const LOG_FILE_NAME: &str = "log.txt";

#[allow(dead_code)]
fn main() -> io::Result<()> {
  let cur_path = env::current_dir().unwrap_or(PathBuf::from(MAIN_SEPARATOR_STR));

  // read log file, create one if it does not exist
  // append log info to file end

  // TODO replace path declarations with parsed config
  // let config_path = Path::new("./config.json");
  // let home_path = Path::new("/home/q");
  // let etc_path = Path::new("/etc");
  // let backup_path = Path::new("/data/backup");

  // let paths: Vec<(PathBuf, PathBuf)> = vec![
  // (home_path.join("bin"), backup_path.join("bin")),
  // (home_path.join(".config"), backup_path.join("config")),
  // (home_path.join(".ssh"), backup_path.join(".ssh")),
  // (home_path.join(".bashrc"), backup_path.join(".bashrc")),
  // (home_path.join(".gitconfig"), backup_path.join(".gitconfig")),
  // (home_path.join(".Xmodmap"), backup_path.join(".Xmodmap")),
  // (etc_path.join("fstab"), backup_path.join("fstab")),
  // (etc_path.join("default/grub"), backup_path.join("grub")),
  // (home_path.join("Documents"), backup_path.join("Documents")),
  // ];

  // for (source_path, target_path) in paths {
  //   let error_msg = format!("Cannot locate the path {:?}", &source_path);
  //
  //   source_path.try_exists().expect(&error_msg);
  //
  //   visit_dirs(&source_path, |dir_entry: &DirEntry| {
  //     // if target_path does not exist create it, or create parent for file
  //     println!("{:#?}", dir_entry.path());
  //     // DirBuilder::new()
  //     //     .recursive(true)
  //     //     .create(path).unwrap();
  //   })
  //   .expect("Failed directory traversal");
  // }

  // Path::try_exists
  // Path::is_file
  // Path::is_dir

  // Command::new("echo").args(["---", ">>", LOGS_FILE.to_str().unwrap()]);
  // Command::new("echo").args([
  //   format!("{:?} Backup complete", SystemTime::now()).as_str(),
  //   ">>",
  //   LOGS_FILE.to_str().unwrap(),
  // ]);
  // TODO Replace echo calls with writing to file

  // BufWriter
  let mut file = File::options()
    .create(true)
    .append(true)
    .open(cur_path.join(LOG_FILE_NAME))
    .expect("Failed reading/creating logs file");
  // let mut buf_reader = BufReader::new(file);
  // let mut contents = String::new();
  // buf_reader.read_to_string(&mut contents)?;

  let mut new_content = String::from("----\n");
  new_content.push_str("Current 'date-time'\n");
  new_content.push_str("Backup complete in 'time', 'count' files are backed up\n");
  write!(file, "{new_content}")?;

  file.sync_all()?;

  Ok(())
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: fn(&DirEntry)) -> io::Result<()> {
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        visit_dirs(&path, cb)?;
      } else {
        cb(&entry);
      }
    }
  }
  Ok(())
}
