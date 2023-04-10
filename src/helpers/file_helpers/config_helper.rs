use std::fs::{self, File, OpenOptions};
use std::io::{self, Error, ErrorKind, Read, Write};
// use toml::Table;
use toml::Value;

pub fn get_config() -> Result<File, io::Error> {
    println!("Finding config file...");
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
                return Err(Error::new(ErrorKind::Other, err));
            }
        },
        Ok(_) => {
            println!("Creating config file...");
            let mut toml_obj = toml::map::Map::new();

            toml_obj.insert("PRIVATE_KEY".to_string(), Value::String("".to_string()));

            let new_config_ref = OpenOptions::new()
                .create(true)
                .write(true)
                .open("config.toml")?;

            let toml_string = toml::to_string(&toml_obj).unwrap();

            fs::write("config.toml", toml_string)?;

            return Ok("Successfully created config file.".to_owned());
        }
    }
}
