use std::io::ErrorKind;
use std::path::{MAIN_SEPARATOR_STR, PathBuf};
use std::{env, fs, io, thread};

use anyhow::anyhow;
use clap::Parser;
use regex::Regex;

use crate::modules::constants::{
  FSErrors, LOG_FILE_NAME, THREAD_POOL_LIMIT, THREAD_POOL_SHARE_OF_CPU_THREADS,
};
use crate::modules::structs::{BackupConfig, CliArgs, CliConfig};

pub fn get_parsed_config(config_path: PathBuf) -> CliConfig {
  if !config_path.exists() {
    panic!(
      "{:#?}",
      FSErrors::NotFound {
        source_path: String::from(config_path.to_str().unwrap()),
        err: io::Error::new(ErrorKind::NotFound, "Failed reading config file")
      }
    );
  }

  let config_data_string =
    fs::read_to_string(&config_path).unwrap_or_else(|err| {
      panic!(
        "{:#?}",
        FSErrors::ReadFileError {
          source_path: String::from(config_path.to_str().unwrap()),
          err
        }
      );
    });

  let config: CliConfig = serde_json::from_str(&config_data_string)
    .unwrap_or_else(|err| {
      panic!(
        "{:#?}",
        FSErrors::OtherError(anyhow!("Failed parsing config file: {:?}", err))
      );
    });

  config
}

pub fn get_backup_config() -> BackupConfig {
  let cli_args = CliArgs::parse();
  let CliArgs {
    config: config_path,
  } = cli_args;
  let config = get_parsed_config(config_path);
  let CliConfig {
    source,
    ignore,
    target,
    log,
  } = config;

  let default_log_path = env::current_dir()
    .unwrap_or(PathBuf::from(MAIN_SEPARATOR_STR))
    .join(LOG_FILE_NAME);
  let log_path = log.unwrap_or(default_log_path);

  let ignore = ignore.map(|ignore| {
    Regex::new(ignore.as_str()).unwrap_or_else(|err| {
      panic!(
        "{:#?}",
        FSErrors::OtherError(anyhow!("Failed parsing regex: {}", err))
      )
    })
  });

  BackupConfig {
    source,
    ignore,
    target,
    log_path,
  }
}

pub fn get_thread_pool_size() -> usize {
  let count = thread::available_parallelism().unwrap_or_else(|err| {
    panic!(
      "{:#?}",
      FSErrors::OtherError(anyhow!(
        "Failed reading number of threads: {}",
        err
      ))
    )
  });

  std::cmp::min(
    (count.get() as f32 / THREAD_POOL_SHARE_OF_CPU_THREADS).floor() as usize,
    THREAD_POOL_LIMIT,
  )
}
