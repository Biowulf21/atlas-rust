use std::fs::File;
use std::io::{self, Read};
// use toml::Table;
// use toml::Value;

pub fn get_config() -> Result<File, io::Error> {
    let config_file = File::open("config.toml")?;
    return Ok(config_file);
}

pub fn create_config() -> Result<String, io::Error> {
    let new_config = File::create("config.toml");
    println!("Creating config file");
    return Ok("Successfully created config file".to_owned());
}
