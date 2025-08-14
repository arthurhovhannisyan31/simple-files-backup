use crate::modules::structs::Config;
use std::fs;
use std::path::PathBuf;

pub fn get_parsed_config(config_path: PathBuf) -> Config {
  if !config_path.exists() {
    panic!("Failed reading config file");
  }

  let config_data_string = fs::read_to_string(config_path)
    .expect("Failed reading config file content");

  let parsed_config: Config = serde_json::from_str(&config_data_string)
    .expect("Failed parsing config file");

  parsed_config
}
