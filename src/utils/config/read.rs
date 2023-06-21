use crate::utils::config::structure::Config;
use crate::utils::config::operate::{from_file, check_exist};
use crate::utils::config::construct::construct;
use log::{info, warn};

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
            construct(file_path);
            info!("Config file created at {}.", file_path);
            info!("Please edit the config file and restart the program.");
            std::process::exit(0);
        } else {
            info!("Config file not created.");
            std::process::exit(0);
        }
    }
}