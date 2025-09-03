use std::path::PathBuf;

use clap::Parser;
use regex::Regex;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, next_line_help = true)]
pub struct CliArgs {
  #[clap(short = 'c', long)]
  pub config: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct CliConfig {
  #[serde(default)]
  pub ignore: Option<String>,
  pub source: Vec<PathBuf>,
  pub target: PathBuf,
  pub log: Option<PathBuf>,
}

pub struct BackupConfig {
  pub ignore: Option<Regex>,
  pub source: Vec<PathBuf>,
  pub target: PathBuf,
  pub log_path: PathBuf,
}
