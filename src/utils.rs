use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(not(debug_assertions))]
use dirs::data_dir;
use lazy_static::lazy_static;
use nannou::text::Font;

use crate::prelude::notify_error;

#[cfg(not(debug_assertions))]
lazy_static! {
    pub static ref BASE_DIR: String = format!(
        "{}/stuff_made_by_lily/GOL",
        data_dir().unwrap().to_str().unwrap()
    );
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
lazy_static! {
    pub static ref BASE_DIR: String = ".".to_string();
}

pub fn load_font() -> Font {
    let asset = include_bytes!("../assets/fonts/jetbrains mono.ttf");
    Font::from_bytes(asset).unwrap()
}

#[derive(Clone, Copy)]
pub struct VecU2 {
    pub x: usize,
    pub y: usize,
}

impl VecU2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Serialize for VecU2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}x{}", self.x, self.y))
    }
}

impl<'de> Deserialize<'de> for VecU2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('x').collect();
        Ok(VecU2 {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        })
    }
}

impl From<(usize, usize)> for VecU2 {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

pub fn fmt_num(num: impl Into<usize> + ToString) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap_or_else(|e| {
            notify_error(format!("Failed to format number: {}.", e));
            "ERROR".split("").collect()
        })
        .join(",")
}
