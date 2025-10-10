use std::io;

use thiserror::Error;

pub const LOG_FILE_NAME: &str = "files-backup-log.txt";
pub const THREAD_POOL_SHARE_OF_CPU_THREADS: f32 = 0.25;
pub const THREAD_POOL_LIMIT: usize = 4;

#[derive(Error, Debug)]
pub enum FSErrors {
  #[error("I/O error occurred")]
  IoError(#[from] io::Error),
  #[error("Failed copying file: `{source_path:?}` `{target_path:?}`")]
  CopyFileError {
    source_path: String,
    target_path: String,
    err: io::Error,
  },
  #[error("Failed creating file: `{target_path:?}`")]
  CreateFileError { target_path: String, err: io::Error },
  #[error("Failed creating link: `{source_path:?}` `{target_path:?}`")]
  CreateSymlinkError {
    source_path: String,
    target_path: String,
    err: io::Error,
  },
  #[error("Failed locating path: `{source_path:?}`")]
  NotFound { source_path: String, err: io::Error },
  #[error("Failed writing logs: `{target_path:?}`")]
  WriteLogsError { target_path: String, err: io::Error },
  #[error("Failed reading file: `{source_path:?}`")]
  ReadFileError { source_path: String, err: io::Error },
  #[error("Failed reading directory: `{source_path:?}`")]
  ReadDirError { source_path: String, err: io::Error },
  #[error("Failed removing file: `{source_path:?}`")]
  RemoveFileError { source_path: String, err: io::Error },
  #[error("File sync error: `{source_path:?}`")]
  FileSyncError { source_path: String, err: io::Error },
  #[error(transparent)]
  OtherError(#[from] anyhow::Error),
}
