use crate::{prelude::*, ui::notify_info};

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

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    fn clear(model: &mut Model) {
        model.selection = None;
    }

    if app.keys.mods.ctrl() {
        match key {
            Key::C => {
                if let Some(selection) = model.selection.take() {
                    selection.copy(model);
                    clear(model);
                }
            }
            Key::X => {
                if let Some(selection) = model.selection.take() {
                    selection.copy(model);
                    selection.clear(model);
                    clear(model);
                }
            }
            Key::S => {
                save_board(model.board.clone());
                notify_info("Board saved to file.");
                clear(model)
            }
            Key::V => {
                if model.clipboard.is_some() {
                    Selection::paste(model);
                }
            }
            _ => (),
        }
    } else {
        match key {
            Key::Delete => {
                if let Some(selection) = model.selection.take() {
                    selection.clear(model);
                }
            }
            Key::W => {
                if let Some(selection) = model.selection.take() {
                    selection.translate(model, 0, 1);
                }
            }
            Key::S => {
                if let Some(selection) = model.selection.take() {
                    selection.translate(model, 0, -1);
                }
            }
            Key::A => {
                if let Some(selection) = model.selection.take() {
                    selection.translate(model, -1, 0);
                }
            }
            Key::D => {
                if let Some(selection) = model.selection.take() {
                    selection.translate(model, 1, 0);
                }
            }
            Key::Q => {
                if let Some(selection) = model.selection.take() {
                    selection.rotate(model, Rotation::CW);
                }
            }
            Key::E => {
                if let Some(selection) = model.selection.take() {
                    selection.rotate(model, Rotation::CCW);
                }
            }
            Key::Space => {
                model.paused = !model.paused;
                clear(model)
            }
            Key::C => {
                model.board.clear();
                clear(model);
            }
            Key::G => model.grid_lines = !model.grid_lines,
            Key::D => {
                model.symmetry = !model.symmetry;
                clear(model);
            }
            Key::F => model.show_info = !model.show_info,
            Key::N => {
                model.board.advance();
                clear(model);
            }
            Key::K => {
                model.show_keybinds = !model.show_keybinds;
                clear(model);
            }
            _ => (),
        }
    }
}
