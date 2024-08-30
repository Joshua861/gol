use crate::prelude::*;

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let cache = &mut model.cache;

    model.fps.tick();

    {
        let smoothing_factor = CONFIG.smoothing_factor / 10.;
        let mut cache_needs_updating = false;

        let mut f = |var: &mut f32, target: f32| {
            if *var != target {
                cache_needs_updating = true;
                *var = *var * (1.0 - smoothing_factor) + target * smoothing_factor
            }
        };

        f(&mut cache.tile_size, cache.target_tile_size);
        f(&mut cache.camera_offset.0, cache.target_camera_offset.0);
        f(&mut cache.camera_offset.1, cache.target_camera_offset.1);

        if cache_needs_updating {
            cache.update(model.board.wh(), cache.tile_size);
        }

        model.cache.tile_size = model.cache.tile_size * (1.0 - smoothing_factor)
            + model.cache.target_tile_size * smoothing_factor;
    }

    if !model.paused {
        time!("advance", { model.board.advance() });
    }
    if let Some(button) = model.pressed {
        if model.selection.is_none() {
            let mut set = |to: bool| {
                let pos = app.mouse.position();
                let (x, y) = pixel_to_board(pos, &model.cache);

                let (board_width, board_height) = model.board.wh();

                let mut set_tile = |x, y, to| {
                    model.board.set(x, y, to);

                    if model.symmetry {
                        let x_mirrored = board_width.saturating_sub(1 + x);
                        model.board.set(x_mirrored, y, to);

                        let y_mirrored = board_height.saturating_sub(1 + y);
                        model.board.set(x, y_mirrored, to);

                        model.board.set(x_mirrored, y_mirrored, to);
                    }
                };

                if model
                    .last_mouse_pressed
                    .is_some_and(|last_button| button == last_button)
                {
                    let (px, py) = pixel_to_board(f32_to_vec2(model.last_mouse_pos), &model.cache);

                    model.board.draw_line(x, y, px, py, to);
                    if model.symmetry {
                        let x_mirrored = (board_width).saturating_sub(1 + x);
                        let px_mirrored = (board_width).saturating_sub(1 + px);
                        let y_mirrored = (board_height).saturating_sub(1 + y);
                        let py_mirrored = (board_height).saturating_sub(1 + py);

                        model.board.draw_line(x_mirrored, y, px_mirrored, py, to);
                        model.board.draw_line(x, y_mirrored, px, py_mirrored, to);
                        model
                            .board
                            .draw_line(x_mirrored, y_mirrored, px_mirrored, py_mirrored, to);
                    }
                } else {
                    set_tile(x, y, to);
                }
            };

            match button {
                MouseButton::Left => set(true),
                MouseButton::Right => set(false),
                _ => (),
            }
        }

        model.last_mouse_pos = vec2_to_f32(app.mouse.position());
        model.last_mouse_pressed = Some(button);
    } else {
        model.last_mouse_pressed = None;
    }
}
