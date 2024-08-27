use lazy_static::lazy_static;
use nannou::color::Srgb;
use serde::de::{self, SeqAccess, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;
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
    pub background_color: Color,
    pub cell_color: Color,
    pub cell_color_highlighted: Color,
    pub background_color_highlighted: Color,
    pub zoom_speed: f32,
    pub grid_color: Color,
    pub grid_thickness: f32,
    pub scale_grid_with_zoom: bool,
}

#[derive(Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of three integers")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let r = seq.next_element::<u8>()?.unwrap_or(0);
        let g = seq.next_element::<u8>()?.unwrap_or(0);
        let b = seq.next_element::<u8>()?.unwrap_or(0);

        Ok(Color::new_u8(r, g, b))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ColorVisitor)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let r = (self.r * 255.0).round() as i64;
        let g = (self.g * 255.0).round() as i64;
        let b = (self.b * 255.0).round() as i64;

        // Serialize as a sequence of three integers
        let color_array = [r, g, b];
        color_array.serialize(serializer)
    }
}

impl Color {
    pub fn to_srgb(&self) -> Srgb {
        Srgb::new(self.r, self.g, self.b)
    }
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    pub fn new_u8(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let text = fs::read_to_string(CONFIG_PATH);

        if let Ok(text) = text {
            return toml::from_str(text.as_str()).unwrap();
        } else {
            println!("Failed to read config file; using default values.");

            #[allow(deprecated)]
            let path = CONFIG_PATH
                .split('/')
                .filter(|s| !s.contains(".toml"))
                .collect::<Vec<&str>>()
                .join("/")
                .replace('~', env::home_dir().unwrap().to_str().unwrap());

            let _ = fs::create_dir_all(path);

            let text = Self::default().to_toml();

            let text = format!(
                r#"# Keybinds:
#    G: Show grid lines,
#    C: Clear grid,
#    Space: Play/pause

{}"#,
                text
            );

            fs::write(CONFIG_PATH, text).expect("Failed to write default values to config file");

            Self::default()
        }
    }
    pub fn default() -> Self {
        Self {
            window_title: String::from("Game of Life"),
            tile_size: 10.0,
            background_color: Color::new(0.1, 0.1, 0.1),
            cell_color: Color::new(0.9, 0.9, 0.9),
            cell_color_highlighted: Color::new(0.8, 0.8, 0.8),
            background_color_highlighted: Color::new(0.2, 0.2, 0.2),
            zoom_speed: 3.0,
            grid_color: Color::new(0.35, 0.35, 0.35),
            grid_thickness: 1.0,
            scale_grid_with_zoom: false,
        }
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}
