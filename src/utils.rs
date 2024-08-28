use std::fs;

#[cfg(not(debug_assertions))]
use dirs::data_dir;
use lazy_static::lazy_static;
use nannou::text::Font;

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

pub fn load_font(name: &str) -> Font {
    Font::from_bytes(fs::read(format!("assets/fonts/{}.ttf", name)).unwrap()).unwrap()
}
