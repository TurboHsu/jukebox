use log::{info, warn};
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use toml::{from_str, to_string};


pub fn construct_default(file_path: &str) {
    // Check whether the config dir exists
    let config_dir = Path::new(file_path).parent().unwrap();
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir).unwrap();
    }
    
    // Load example configuration file
    let content = include_bytes!("../../statics/example.toml");


    // Write this string to the file
    let mut file = File::create(Path::new(file_path)).unwrap();
    file.write_all(content).unwrap();
}

// This file is used to read the config file
pub fn from_file(file_path: &str) -> Config {
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

    config
}

// This function is used to check if the file exists
pub fn check_exist(file_path: &str) -> bool {
    return Path::new(file_path).exists();
}

// This function is used to write the config file
fn write_config(config: Config, file_path: &str) {
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

// Reads the config file, if it doesn't exist, creates it.
pub fn read_config(file_path: &str) -> Config {
    if check_exist(file_path) {
        from_file(file_path)
    } else {
        warn!("Config file does not exist, do you want to create a template config at {}? (y/N)", file_path);
        // Get decision from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "y" || input == "Y" || input == "yes" || input == "Yes" {
            construct_default(file_path);
            info!("Config file created at {}.", file_path);
            info!("Please edit the config file and restart the program.");
            std::process::exit(0);
        } else {
            info!("Config file not created.");
            std::process::exit(0);
        }
    }
}


// Structure of configuration
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub general: General,
    pub libraries: Vec<Library>,
    pub lyrics: Lyrics,
}

#[derive(Serialize, Deserialize)]
pub struct General {
    pub debug: bool,
    pub config_directory: String,
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub location: String,
}

#[derive(Serialize, Deserialize)]
pub struct Lyrics {
    pub use_engine: Vec<String>,
}