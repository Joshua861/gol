use game_logic::{model, update, view};

mod config;
mod game;
mod game_logic;
mod prelude;
mod savestates;
mod timing;
mod ui;
mod utils;

fn main() {
    nannou::app(model).update(update).view(view).run();
}
