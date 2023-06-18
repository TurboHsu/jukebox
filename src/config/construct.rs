use crate::config::structure::{Config, General, Library, Lyrics};

pub fn construct(file_path: &str) -> Config {
    // WIP
    return Config {
        general: General {
            debug: false,
            config_directory: "WIP".to_string(),
        },
        libraries: vec![Library {
            name: "WIP".to_string(),
            location: "WIP".to_string(),
        }],
        lyrics: Lyrics {
            use_engine: vec!["WIP".to_string()],
        },
    };
}