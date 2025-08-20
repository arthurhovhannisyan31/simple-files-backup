use std::fs::DirBuilder;
use std::io;
use std::sync::mpsc;
use std::time::Instant;

pub mod modules;
use modules::structs::BackupConfig;
use modules::types::{BackupCommand, BackupResult};
use modules::utils::config::get_backup_config;
use modules::utils::config::get_thread_pool_size;
use modules::utils::logs::write_logs;
use modules::utils::threads::{backup_files, spawn_backup_threads};

fn main() -> io::Result<()> {
  let start_time = Instant::now();
  let mut files_count: usize = 0;
  let threads_count = get_thread_pool_size();
  let backup_config = get_backup_config();
  let BackupConfig {
    source,
    ignore,
    target,
  } = backup_config;

  if !target.exists() {
    DirBuilder::new().recursive(true).create(&target)?;
  }

  let (command_sender, command_receiver) = mpsc::channel::<BackupCommand>();
  let (result_sender, result_receiver) = mpsc::channel::<BackupResult>();

  spawn_backup_threads(command_receiver, result_sender, threads_count);
  let mut log_message = backup_files(
    command_sender,
    result_receiver,
    source,
    target,
    ignore,
    &mut files_count,
  );

  log_message.push_str(&format!(
    "Backup complete in {:?}, {} files were backed up\n",
    start_time.elapsed(),
    files_count
  ));

  write_logs(&log_message)?;
  Ok(())
}
