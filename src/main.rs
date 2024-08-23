use std::{
    io::{self, Write},
    time,
};

use config::CONFIG;
use game::Game;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

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
        panic!("Failed to create window: {}", e);
    });

    window.set_target_fps(CONFIG.target_fps);

    let mut game = Game::new();
    let mut paused = false;
    let mut out = io::stdout();

    let x_multiplier = CONFIG.board_width as f32 / CONFIG.window_width as f32;
    let y_multiplier = CONFIG.board_height as f32 / CONFIG.window_height as f32;

    let mapping: Vec<(usize, usize)> = (0..buffer.len())
        .map(|i| {
            let px = i % CONFIG.window_width;
            let py = i / CONFIG.window_width;
            let x = (px as f32 * CONFIG.board_width as f32 / CONFIG.window_width as f32) as usize;
            let y = (py as f32 * CONFIG.board_height as f32 / CONFIG.window_height as f32) as usize;
            (x, y)
        })
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            paused = !paused;
        }

        if !paused {
            let timer = time::Instant::now();
            game.advance();

            buffer
                .par_iter_mut()
                .zip(mapping.par_iter())
                .for_each(|(v, &(x, y))| {
                    *v = if game.get(x, y).unwrap_or(false) {
                        0xFFF5F5F5
                    } else {
                        0xFF0A0A0A
                    };
                });

            window
                .update_with_buffer(&buffer, CONFIG.window_width, CONFIG.window_height)
                .unwrap();

            print!("\rFPS: {:.1}", 1.0 / timer.elapsed().as_secs_f32());
            let _ = out.flush();
        }

        window.update();
    }
}
