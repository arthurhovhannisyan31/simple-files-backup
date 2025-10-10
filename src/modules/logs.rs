use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use chrono::Local;

use crate::modules::constants::FSErrors;

pub fn write_logs(log_path: &PathBuf, log_str: &str) {
  let mut file = File::options()
    .create(true)
    .append(true)
    .open(log_path)
    .unwrap_or_else(|err| {
      panic!(
        "{:#?}",
        FSErrors::NotFound {
          source_path: String::from(log_path.to_str().unwrap()),
          err
        }
      )
    });

  let mut new_content = String::from("----\n");
  new_content.push_str(format!("{}\n", Local::now().to_rfc2822()).as_str());
  new_content.push_str(log_str);

  file
    .write_all(new_content.as_bytes())
    .unwrap_or_else(|err| {
      panic!(
        "{:#?}",
        FSErrors::WriteLogsError {
          target_path: String::from(log_path.to_str().unwrap()),
          err
        }
      )
    });
  file.sync_all().unwrap_or_else(|err| {
    panic!(
      "{:#?}",
      FSErrors::FileSyncError {
        source_path: String::from(log_path.to_str().unwrap()),
        err
      }
    );
  });
}
