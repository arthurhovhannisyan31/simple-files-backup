use clap::Parser;
use regex::Regex;
use std::fs::DirBuilder;
use std::path::{MAIN_SEPARATOR_STR, PathBuf};
use std::time::Instant;
use std::{env, io};

pub mod modules;
use modules::structs::{CliArgs, Config};
use modules::utils::{
  config::get_parsed_config, dirs::traverse_paths, logs::write_logs,
};

fn main() -> io::Result<()> {
  let start_time = Instant::now();
  let cli_args = CliArgs::parse();
  let CliArgs {
    config: config_path,
  } = cli_args;
  let config = get_parsed_config(config_path);
  let mut files_count: usize = 0;
  let Config {
    source,
    ignore,
    target,
  } = config;
  let ignore = ignore.map(|ignore| {
    Regex::new(ignore.as_str())
      .unwrap_or_else(|_| panic!("Failed parsing regex"))
  });

  if !target.exists() {
    DirBuilder::new().recursive(true).create(&target)?;
  }

  let log_message =
    match traverse_paths(source, ignore.as_ref(), target, &mut files_count) {
      Ok(_) => format!(
        "Backup complete in {:?}, {} files were backed up\n",
        start_time.elapsed(),
        files_count
      ),
      Err(err) => format!("{err:?}"),
    };

  let cur_path =
    env::current_dir().unwrap_or(PathBuf::from(MAIN_SEPARATOR_STR));

  write_logs(&cur_path, &log_message)?;

  Ok(())
}
