use std::error::Error;
use std::fs;

use crate::data::bfc::Bfc;
use crate::config::config::Config;

pub fn read_bfcs_from_json(json_path: &String) -> Result<Vec<Bfc>, Box<dyn Error>> {
    let file: String = fs::read_to_string(json_path.to_string())?.parse()?;
    let datas: Vec<Bfc> = serde_json::from_str(&file.as_str())
        .expect("Error in read file, fomat invalid");
    Ok(datas)
}

pub fn read_config_from_json(json_path: &String) -> Result<Config, Box<dyn Error>>{
    let file: String = fs::read_to_string(json_path.to_string())?.parse()?;
    let datas: Config = serde_json::from_str(&file.as_str())
        .expect("Error in read file, fomat invalid");
    Ok(datas)
}