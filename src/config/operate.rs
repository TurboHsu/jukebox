use crate::config::structure::Config;

// This file is used to read the config file
pub fn from_file(file_path: &str) -> Config {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use std::process::exit;
    use toml::from_str;
    // Read the file
    let mut file = File::open(Path::new(file_path)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // Parse the file
    let config: Config = match from_str(&content) {
        Ok(config) => config,
        Err(err) => {
            println!("The config file is not valid.");
            println!("{}", err);
            exit(1);
        }
    };

    return config;
}

// This function is used to check if the file exists
pub fn check_exist(file_path: &str) -> bool {
    use std::path::Path;
    return Path::new(file_path).exists();
}

// This function is used to write the config file
fn write_file(config: Config, file_path: &str) {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::process::exit;
    use toml::to_string;

    // Parse the config to string
    let content = match to_string(&config) {
        Ok(content) => content,
        Err(err) => {
            println!("The config file is not valid.");
            println!("{}", err);
            exit(1);
        }
    };

    // Write the file
    let mut file = File::create(Path::new(file_path)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}