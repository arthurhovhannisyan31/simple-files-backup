use std::fs::DirBuilder;
use std::io;
use std::time::Instant;

pub mod modules;
use modules::backup::backup;
use modules::config::get_backup_config;
use modules::config::get_thread_pool_size;
use modules::logs::write_logs;
use modules::structs::BackupConfig;

fn main() -> io::Result<()> {
  let start_time = Instant::now();
  let mut files_count: usize = 0;
  let threads_count = get_thread_pool_size();

  let backup_config = get_backup_config();
  let BackupConfig {
    source,
    ignore,
    target,
    log_path,
  } = backup_config;

  if !target.exists() {
    DirBuilder::new().recursive(true).create(&target)?;
  }

  let mut log_message =
    backup(source, target, ignore, &mut files_count, threads_count);

  log_message.push_str(&format!(
    "Backup complete in {:?}, {} files were backed up\n",
    start_time.elapsed(),
    files_count
  ));

  write_logs(log_path, &log_message)?;
  Ok(())
}
