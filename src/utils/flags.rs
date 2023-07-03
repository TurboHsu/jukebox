use clap::Parser;
use lazy_static::lazy_static;


lazy_static! {
    static ref ARGS: Args = initiate();
}

fn initiate() -> Args {
    Args::parse()
}

// This function is used to get lazy cached arguments
pub fn get_arg() -> &'static Args {
    &*ARGS
}


// Structure
#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = Args::get_default_config_file())]
    pub config_file: String,
}

impl Args {
    fn get_default_config_file() -> &'static str {
        let config_dir = dirs::config_dir().unwrap();
        let config_dir = config_dir.to_str().unwrap();
        let config_dir: String = format!("{}{}{}{}{}", config_dir, std::path::MAIN_SEPARATOR, "jukebox",
            std::path::MAIN_SEPARATOR, "config.toml");
        Box::leak(config_dir.into_boxed_str())
    }
}

