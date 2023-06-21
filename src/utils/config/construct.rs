use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn construct(file_path: &str) {
    // Check whether the config dir exists
    let config_dir = Path::new(file_path).parent().unwrap();
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir).unwrap();
    }
    
    // Creates the file
    let config_template = "
    Something should be here";
    // Write this string to the file
    let mut file = File::create(Path::new(file_path)).unwrap();
    file.write_all(config_template.as_bytes()).unwrap();
}