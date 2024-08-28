use super::*;
use crate::{savestates, time};

pub fn window_resized(_app: &App, model: &mut Model, rect: Vec2) {
    time!("window_resize", {
        let width = (rect.x / CONFIG.tile_size).ceil() as usize;
        let height = (rect.y / CONFIG.tile_size).ceil() as usize;
        model.board.set_wh(width, height);

        model.cache.update((width, height), CONFIG.tile_size);
        model.cache.window_size = (rect.x, rect.y);
        model.cache.camera_offset = (0., 0.);
    });
}

pub fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.paused = !model.paused,
        Key::C => model.board.clear(),
        Key::G => model.grid_lines = !model.grid_lines,
        Key::S => savestates::save(model.board.clone()),
        Key::D => model.symmetry = !model.symmetry,
        Key::F => model.show_info = !model.show_info,
        Key::N => model.board.advance(),
        _ => (),
    }
}
