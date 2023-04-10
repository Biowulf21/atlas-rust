use crate::helpers::file_helpers::config_helper;

pub fn init_app() {
    let contents_or_error = config_helper::get_config();

    let finding_config_result = match contents_or_error {
        Ok(file) => {
            println!("Found config file. Proceeding.");
            println!("{:?}", file);
            file;
        }
        Err(err) => {
            println!("Config file not found. Creating one.");
            let creation_result = config_helper::create_config();
        }
    };
    println!("{:?}", finding_config_result);
}
