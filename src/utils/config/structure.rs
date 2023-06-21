use serde_derive::{Serialize, Deserialize};

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