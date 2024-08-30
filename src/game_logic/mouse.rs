use super::{clamp_camera, pixel_to_board, Model, Selection};
use crate::config::CONFIG;
use nannou::prelude::*;

pub fn mouse_wheel(_app: &App, model: &mut Model, delta: MouseScrollDelta, _phase: TouchPhase) {
    match delta {
        MouseScrollDelta::LineDelta(_, y) => {
            let cache = &mut model.cache;
            let mult = y * CONFIG.zoom_speed * cache.scale_factor;
            cache.target_tile_size += mult;

            clamp_camera(model);
        }
        MouseScrollDelta::PixelDelta(_) => {}
    }
}

pub fn mouse_moved(app: &App, model: &mut Model, pos: Vec2) {
    model.mouse_pos = (pos.x, pos.y);

    if let Some(selection) = &mut model.selection {
        if app.keys.mods.ctrl() && app.mouse.buttons.left().is_down() {
            selection.end = pixel_to_board(pos, &model.cache).into();
        }
    }

    if model.last_mouse_pressed == Some(MouseButton::Middle) {
        let (dx, dy) = (
            model.last_mouse_pos.0 - pos.x,
            model.last_mouse_pos.1 - pos.y,
        );

        let cache = &mut model.cache;
        let camera_offset = &mut cache.target_camera_offset;
        let f = |offset: f32, change: f32| -> f32 {
            offset - (change * CONFIG.pan_speed / 100.) / cache.scale_factor
        };
        camera_offset.0 = f(camera_offset.0, dx);
        camera_offset.1 = f(camera_offset.1, dy);

        clamp_camera(model);
    }
}

pub fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    model.pressed = Some(button);

    if app.keys.mods.ctrl() {
        if let MouseButton::Left = button {
            let (x, y) = pixel_to_board(app.mouse.position(), &model.cache);
            model.selection = Some(Selection::new(x, y));
        }
    } else {
        model.selection = None;
    }
}

pub fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.pressed = None;
}
