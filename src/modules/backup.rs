use std::path::PathBuf;
use std::sync::mpsc;

use regex::Regex;

use crate::modules::threads::spawn_backup_threads;
use crate::modules::traverse::traverse_sources;
use crate::modules::types::{BackupCommand, BackupResult};

pub fn backup(
  source: Vec<PathBuf>,
  target: PathBuf,
  ignore: Option<Regex>,
  files_count: &mut usize,
  threads_count: usize,
) -> String {
  let (command_sender, command_receiver) = mpsc::channel::<BackupCommand>();
  let (result_sender, result_receiver) = mpsc::channel::<BackupResult>();
  let mut error_message = String::new();

  spawn_backup_threads(command_receiver, result_sender, threads_count);
  if let Err(err) =
    traverse_sources(command_sender, source, target, ignore.as_ref())
  {
    error_message.push_str(&err.to_string());
  }

  for msg in result_receiver.iter() {
    match msg {
      Ok(_) => {
        *files_count += 1;
      }
      Err(err) => {
        error_message.push_str(&format!("{:?}\n", err));
      }
    }
  }

  error_message
}
