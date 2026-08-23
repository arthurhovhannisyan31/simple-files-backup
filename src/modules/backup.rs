use std::fs::{remove_dir_all, rename};
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
  let mut pending_swaps: Vec<(PathBuf, PathBuf)> = Vec::new();

  spawn_backup_threads(command_receiver, result_sender, threads_count);

  match traverse_sources(command_sender, source, target, ignore.as_ref()) {
    Ok(swaps) => pending_swaps = swaps,
    Err(err) => error_message.push_str(&err.to_string()),
  }

  // Blocks until every worker has finished copying (all senders dropped).
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

  // All copies are now complete: atomically swap each staged directory over
  // its final destination. This is the first point at which any previous
  // backup is removed, so an interrupted run never destroys the old copy.
  for (staging_path, final_path) in pending_swaps {
    if final_path.exists() {
      if let Err(err) = remove_dir_all(&final_path) {
        error_message.push_str(&format!(
          "Failed removing previous backup `{}`: {err}\n",
          final_path.to_string_lossy()
        ));
        continue;
      }
    }
    if let Err(err) = rename(&staging_path, &final_path) {
      error_message.push_str(&format!(
        "Failed swapping staged backup `{}` into `{}`: {err}\n",
        staging_path.to_string_lossy(),
        final_path.to_string_lossy()
      ));
    }
  }

  error_message
}
