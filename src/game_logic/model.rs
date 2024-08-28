use super::*;
use crate::{game::Board, savestates, utils::load_font};
use clap::Parser;
use fps_ticker::Fps;
use nannou::text::Font;

pub struct Model {
    pub board: Board,
    pub paused: bool,
    pub pressed: Option<MouseButton>,
    pub last_mouse_pos: (f32, f32),
    pub last_mouse_pressed: Option<MouseButton>,
    pub cache: Cache,
    pub mouse_pos: (f32, f32),
    pub grid_lines: bool,
    pub symmetry: bool,
    pub show_fps: bool,
    pub fps: Fps,
    pub font: Font,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    load: Option<String>,
}

pub fn model(app: &App) -> Model {
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .resized(window_resized)
        .key_pressed(key_pressed)
        .mouse_moved(mouse_moved)
        .mouse_wheel(mouse_wheel)
        .build()
        .unwrap();

    let initial_tile_size = CONFIG.tile_size;
    let board_size = (1, 1);

    let args = Args::parse();
    let mut board = Board::new(1, 1);
    let mut paused = false;

    if args.load.is_some() {
        board = savestates::load(args.load.unwrap());
        paused = true;
    }

    Model {
        board,
        paused,
        pressed: None,
        last_mouse_pos: (0., 0.),
        cache: Cache::new(board_size, initial_tile_size),
        mouse_pos: (0.0, 0.0),
        grid_lines: false,
        symmetry: false,
        last_mouse_pressed: None,
        show_fps: false,
        fps: Fps::default(),
        font: load_font("jetbrains mono"),
    }
}
