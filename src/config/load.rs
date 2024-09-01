use std::{env, fs};

use crate::{prelude::notify_info, utils::BASE_DIR};

use super::Config;

pub fn load() -> Config {
    let config_path = config_path();

    let text = fs::read_to_string(&config_path);

    if let Ok(text) = text {
        return toml::from_str(text.as_str()).unwrap_or(Config::default());
    } else {
        notify_info("Failed to read config file; using default values.");

        #[allow(deprecated)]
        let path = config_path
            .split('/')
            .filter(|s| !s.contains(".toml"))
            .collect::<Vec<&str>>()
            .join("/")
            .replace('~', env::home_dir().unwrap().to_str().unwrap());

        let _ = fs::create_dir_all(path);

        let text = Config::default().to_toml();

        fs::write(config_path, text).expect("Failed to write default values to config file");

        Config::default()
    }
}

fn config_path() -> String {
    let path = BASE_DIR.to_string() + "/config.toml";
    dbg!(&path);

    path
}
