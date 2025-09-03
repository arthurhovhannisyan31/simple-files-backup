use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use chrono::Local;

pub fn write_logs(log_path: PathBuf, log_str: &str) -> io::Result<()> {
  let mut file = File::options()
    .create(true)
    .append(true)
    .open(log_path)
    .expect("Failed reading/creating log file");

  let mut new_content = String::from("----\n");
  new_content.push_str(format!("{}\n", Local::now().to_rfc2822()).as_str());
  new_content.push_str(log_str);

  file.write_all(new_content.as_bytes())?;
  file.sync_all()?;

  Ok(())
}
