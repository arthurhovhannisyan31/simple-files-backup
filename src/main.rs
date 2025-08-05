use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::path::{MAIN_SEPARATOR_STR, PathBuf};
use std::time::Instant;
use std::{env, io};

const LOG_FILE_NAME: &str = "log.txt";

#[allow(dead_code)]
fn main() -> io::Result<()> {
  let start_time = Instant::now();
  let cur_path = env::current_dir().unwrap_or(PathBuf::from(MAIN_SEPARATOR_STR));

  write_logs(cur_path, start_time, 0)?;

  Ok(())
}

fn write_logs(cur_path: PathBuf, start_time: Instant, files_count: u32) -> io::Result<()> {
  let mut file = File::options()
    .create(true)
    .append(true)
    .open(cur_path.join(LOG_FILE_NAME))
    .expect("Failed reading/creating log file");

  let mut new_content = String::from("----\n");
  new_content.push_str(format!("{}\n", Local::now().to_rfc2822()).as_str());
  new_content.push_str(
    format!(
      "Backup complete in {:?}, {files_count} files were backed up\n",
      start_time.elapsed(),
    )
    .as_str(),
  );

  file.write_all(new_content.as_bytes())?;
  file.sync_all()?;

  Ok(())
}
