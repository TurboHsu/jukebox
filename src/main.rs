mod utils;

fn main() {
    // Testing
    simple_logger::SimpleLogger::new().init().unwrap();
    let args = crate::utils::flags::get();
    println!("{}", args.config_file);
    crate::utils::config::read(&args.config_file);
}