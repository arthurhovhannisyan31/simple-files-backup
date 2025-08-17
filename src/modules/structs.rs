use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, next_line_help = true)]
pub struct CliArgs {
  #[clap(short = 'c', long)]
  pub config: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default)]
  pub ignore: Option<String>,
  pub source: Vec<PathBuf>,
  pub target: PathBuf,
}
