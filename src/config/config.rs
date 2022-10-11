use serde::{Deserialize};
use crate::data::input::json::read_config_from_json;
use serde::de::StdError;


#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub git_cloud_base_url: String,
    pub path_projects: String,
    pub work_directory: String
}

const CONFIG_FILE_PATH: &str = "config.json";

impl Config {
    pub fn new() -> Result<Config, Box<dyn StdError>> {
        let config = read_config_from_json(&CONFIG_FILE_PATH.to_string())?;
        Ok(config)
    }
}