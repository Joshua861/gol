use crate::prelude::*;

pub fn window_resized(_app: &App, model: &mut Model, rect: Vec2) {
    if CONFIG.autosize_board {
        time!("window_resize", {
            let width = (rect.x / CONFIG.tile_size).ceil() as usize;
            let height = (rect.y / CONFIG.tile_size).ceil() as usize;
            model.board.set_wh(width, height);

            model.cache.update((width, height), CONFIG.tile_size);
            model.cache.camera_offset = (0., 0.);
        });
    }

    model.cache.window_size = (rect.x, rect.y);
}
