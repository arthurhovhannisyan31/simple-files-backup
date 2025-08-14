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
  pub ignore: String,
  pub source: Vec<PathBuf>,
  // TODO Serde optional target: Option<PathBuf>
  pub target: PathBuf,
}
