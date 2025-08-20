use std::path::PathBuf;

pub type BackupCommand = (PathBuf, PathBuf);
pub type BackupResult = Result<(), String>;
