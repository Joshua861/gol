use std::{
    io::{self, Write},
    time,
};

use config::CONFIG;
use game::Game;
use log::error;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

mod config;
mod game;
mod utils;

fn main() {
    let mut buffer: Vec<u32> = vec![0; CONFIG.window_width * CONFIG.window_height];

    let mut window = Window::new(
        &(CONFIG.window_title),
        CONFIG.window_width,
        CONFIG.window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        error!("Failed to create window: {}", e);
        panic!();
    });

    window.set_target_fps(CONFIG.target_fps);

    let mut game = Game::new();
    let mut paused = false;
    let mut out = io::stdout();

    let x_multiplier = CONFIG.board_width as f32 / CONFIG.window_width as f32;
    let y_multiplier = CONFIG.board_height as f32 / CONFIG.window_height as f32;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            paused = !paused;
        }

        if !paused {
            let timer = time::Instant::now();

            game.advance();

            buffer.par_iter_mut().enumerate().for_each(|(i, v)| {
                let (px, py) = (i % CONFIG.window_width, i / CONFIG.window_width);

                let (x, y) = (
                    (px as f32 * x_multiplier) as usize,
                    (py as f32 * y_multiplier) as usize,
                );

                let b = if game.get(x, y).unwrap_or(false) {
                    245
                } else {
                    10
                };
                *v = b + (b * 256) + (b * 256 * 256);
            });

            window
                .update_with_buffer(&buffer, CONFIG.window_width, CONFIG.window_height)
                .unwrap();

            print!(
                "\rFPS: {}",
                ((1.0 / timer.elapsed().as_secs_f32()) * 10.).floor() / 10.
            );
            let _ = out.flush();
        }
    }
}
