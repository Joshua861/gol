use color::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use self::load::load;

mod color;
mod load;

#[cfg(target_family = "unix")]
#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "./config.toml";
#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "~/.local/share/stuff_made_by_lily/GOL/config.toml";

#[cfg(target_family = "windows")]
const CONFIG_PATH: &str = ".\\config.toml";

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub window_title: String,
    pub tile_size: f32,
    pub background_color: Color,
    pub void_color: Color,
    pub cell_color: Color,
    pub cell_color_highlighted: Color,
    pub background_color_highlighted: Color,
    pub zoom_speed: f32,
    pub grid_color: Color,
    pub grid_thickness: f32,
    pub scale_grid_with_zoom: bool,
    pub pan_speed: f32,
    pub text_color: Color,
    pub smoothing_factor: f32,
}

impl Config {
    pub fn load() -> Self {
        load()
    }
    pub fn default() -> Self {
        Self {
            window_title: String::from("Game of Life"),
            tile_size: 10.0,
            background_color: Color::new(0.1, 0.1, 0.1),
            cell_color: Color::new(0.9, 0.9, 0.9),
            cell_color_highlighted: Color::new(0.8, 0.8, 0.8),
            background_color_highlighted: Color::new(0.2, 0.2, 0.2),
            zoom_speed: 1.0,
            grid_color: Color::new(0.35, 0.35, 0.35),
            grid_thickness: 1.0,
            scale_grid_with_zoom: false,
            pan_speed: 5.,
            void_color: Color::new(0.08, 0.08, 0.08),
            text_color: Color::new(0.95, 0.95, 0.95),
            smoothing_factor: 3.0,
        }
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}
