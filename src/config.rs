use lazy_static::lazy_static;
use log::warn;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "./config.toml";
#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "~/.local/share/stuff_made_by_lily/GOL/config.toml";

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub window_title: String,
    pub tile_size: f32,
}

impl Config {
    pub fn load() -> Self {
        let text = fs::read_to_string(CONFIG_PATH);

        if let Ok(text) = text {
            return toml::from_str(text.as_str()).unwrap();
        } else {
            warn!("Failed to read config file; using default values.");

            #[allow(deprecated)]
            let path = CONFIG_PATH
                .split('/')
                .filter(|s| !s.contains(".toml"))
                .collect::<Vec<&str>>()
                .join("/")
                .replace('~', env::home_dir().unwrap().to_str().unwrap());

            dbg!(&path);

            let _ = fs::create_dir_all(path);

            fs::write(CONFIG_PATH, Self::default().to_toml())
                .expect("Failed to write default values to config file");

            Self::default()
        }
    }
    pub fn default() -> Self {
        Self {
            window_title: String::from("Game of Life"),
            tile_size: 10.0,
        }
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}
