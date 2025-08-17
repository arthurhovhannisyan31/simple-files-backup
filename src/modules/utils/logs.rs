use crate::modules::configs::constants::LOG_FILE_NAME;
use chrono::Local;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn write_logs(cur_path: &Path, log_str: &str) -> io::Result<()> {
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
