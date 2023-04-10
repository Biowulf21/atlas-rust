use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};
// use toml::Table;
// use toml::Value;

pub fn get_config() -> Result<File, io::Error> {
    let config_file = File::open("config.toml")?;
    return Ok(config_file);
}

pub fn create_config() -> Result<String, io::Error> {
    let new_config = File::create("config.toml");

    match new_config {
        Err(err) => match err.kind() {
            io::ErrorKind::AlreadyExists => {
                return Err(Error::new(ErrorKind::AlreadyExists, "File already exists"));
            }
            io::ErrorKind::PermissionDenied => {
                return Err(Error::new(ErrorKind::PermissionDenied, "Permission denied"));
            }
            _ => {
                return Err(Error::new(ErrorKind::Other, "Unknown error: {err}"));
            }
        },
        Ok(_) => {
            println!("Creating config file");
            return Ok("Successfully created config file".to_owned());
        }
    }
}
