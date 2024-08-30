use game_logic::{model, update, view};
use rust_embed::RustEmbed;

mod config;
mod game;
mod game_logic;
mod prelude;
mod savestates;
mod timing;
mod ui;
mod utils;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

fn main() {
    nannou::app(model).update(update).view(view).run();
}
