use lazy_static::lazy_static;
use log::warn;
use serde::{Deserialize, Serialize};
use std::fs;

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
    pub window_width: usize,
    pub window_height: usize,
    pub target_fps: usize,
    pub board_width: usize,
    pub board_height: usize,
    pub start_paused: bool,
}

impl Config {
    pub fn load() -> Self {
        let text = fs::read_to_string(CONFIG_PATH);

        if let Ok(text) = text {
            return toml::from_str(text.as_str()).unwrap();
        } else {
            warn!("Failed to read config file; using default values.");

            fs::write(CONFIG_PATH, Self::default().to_toml())
                .expect("Failed to write default values to config file.");

            Self::default()
        }
    }
    pub fn default() -> Self {
        Self {
            window_title: String::from("Game of Life"),
            window_width: 1920,
            window_height: 1080,
            target_fps: 60,
            board_height: 192,
            board_width: 108,
            start_paused: true,
        }
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}
