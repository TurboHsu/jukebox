mod utils;

fn main() {
    // Testing
    simple_logger::SimpleLogger::new().init().unwrap();
    let args = utils::flags::get_arg();
    let config = utils::config::read_config(&args.config_file);
    println!("{}", config.general.config_directory) 
}