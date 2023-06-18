use crate::config::structure::Config;
use crate::config::operate::{from_file, check_exist};
use crate::config::construct::construct;

// Reads the config file, and if it doesn't exist, creates it.
pub fn read(file_path: &str) -> Config {
    if check_exist(file_path) {
        return from_file(file_path);
    } else {
        return construct(file_path);
    }
}