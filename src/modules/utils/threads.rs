use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use regex::Regex;

use crate::modules::types::{BackupCommand, BackupResult};
use crate::modules::utils::dirs::traverse_files;
use crate::modules::utils::files::{backup_file, backup_symlink};

pub fn spawn_backup_threads(
  command_receiver: mpsc::Receiver<BackupCommand>,
  result_sender: mpsc::Sender<BackupResult>,
  threads_count: usize,
) {
  let command_receiver = Arc::new(Mutex::new(command_receiver));

  for _ in 0..threads_count {
    let result_sender = result_sender.clone();
    let command_receiver = command_receiver.clone();

    thread::spawn(move || {
      loop {
        let command_result = {
          let receiver_guard = command_receiver.lock().unwrap();
          receiver_guard.recv()
        };
        let Ok((source_path, target_path)) = command_result else {
          break;
        };

        let mut backup_result: BackupResult = Ok(());

        if source_path.is_file() {
          if let Err(err) = backup_file(&source_path, &target_path) {
            backup_result = Err(err.to_string());
          }
        } else if source_path.is_symlink() {
          if let Err(err) = backup_symlink(&source_path, &target_path) {
            backup_result = Err(err.to_string());
          }
        }

        result_sender.send(backup_result).unwrap();
      }
    });
  }
}

pub fn backup_files(
  command_sender: mpsc::Sender<BackupCommand>,
  result_receiver: mpsc::Receiver<BackupResult>,
  source: Vec<PathBuf>,
  target: PathBuf,
  ignore: Option<Regex>,
  files_count: &mut usize,
) -> String {
  let mut error_message = String::new();

  if let Err(err) =
    traverse_files(command_sender, source, target, ignore.as_ref())
  {
    error_message.push_str(&err.to_string());
  }

  for msg in result_receiver.iter() {
    match msg {
      Ok(_) => {
        *files_count += 1;
      }
      Err(err) => {
        error_message.push_str(&format!("Failed copying: {:?}\n", err));
      }
    }
  }

  error_message
}
