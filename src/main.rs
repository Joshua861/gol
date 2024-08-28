use game_logic::{model, update, view};

mod config;
mod game;
mod game_logic;
mod savestates;
mod timing;
mod utils;

fn main() {
    nannou::app(model).update(update).view(view).run();
}
