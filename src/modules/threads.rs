use std::sync::{Arc, Mutex, mpsc};
use std::{fs, thread};

use crate::modules::files::{backup_file, backup_symlink};
use crate::modules::types::{BackupCommand, BackupResult};

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
        let metadata = fs::symlink_metadata(&source_path);

        if let Ok(meta) = metadata {
          if meta.is_symlink() {
            if let Err(err) = backup_symlink(&source_path, &target_path) {
              backup_result = Err(err.to_string());
            }
          } else if meta.is_file() {
            if let Err(err) = backup_file(&source_path, &target_path) {
              backup_result = Err(err.to_string());
            }
          }
        } else {
          backup_result = Err(metadata.unwrap_err().to_string());
        };

        result_sender.send(backup_result).unwrap();
      }
    });
  }
}
