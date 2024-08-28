mod board;
mod config;
mod game_logic;
mod savestates;
mod utils;

use crate::game_logic::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}
