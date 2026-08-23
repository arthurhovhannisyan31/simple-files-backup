use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use chrono::Local;

/// Writes a log entry to `log_path`. Logging failures must never abort the
/// backup, so any error is reported to stderr instead of panicking (a panic
/// here would mask the very error we were asked to record).
pub fn write_logs(log_path: &PathBuf, log_str: &str) {
  let mut new_content = String::from("----\n");
  new_content.push_str(format!("{}\n", Local::now().to_rfc2822()).as_str());
  new_content.push_str(log_str);

  if let Err(err) = try_write_logs(log_path, &new_content) {
    eprintln!(
      "Failed writing logs to `{}`: {err}\n{new_content}",
      log_path.to_string_lossy()
    );
  }
}

fn try_write_logs(log_path: &PathBuf, content: &str) -> std::io::Result<()> {
  let mut file = File::options().create(true).append(true).open(log_path)?;
  file.write_all(content.as_bytes())?;
  file.sync_all()?;
  Ok(())
}
