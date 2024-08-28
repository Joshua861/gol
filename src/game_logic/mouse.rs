use super::Model;
use crate::{clamp_camera, config::CONFIG};
use nannou::prelude::*;

pub fn mouse_wheel(_app: &App, model: &mut Model, delta: MouseScrollDelta, _phase: TouchPhase) {
    match delta {
        MouseScrollDelta::LineDelta(_, y) => {
            let mult = y * CONFIG.zoom_speed * model.cache.scale_factor;
            model.cache.target_tile_size += mult;
            // model.cache.camera_offset.0 += mult;
            // model.cache.camera_offset.1 += mult;

            clamp_camera(model);
        }
        MouseScrollDelta::PixelDelta(_) => {}
    }
}

pub fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    model.mouse_pos = (pos.x, pos.y);

    let (dx, dy) = (
        model.last_mouse_pos.0 - pos.x,
        model.last_mouse_pos.1 - pos.y,
    );
    if model.last_mouse_pressed == Some(MouseButton::Middle) {
        let cache = &mut model.cache;
        let camera_offset = &mut cache.target_camera_offset;
        let f = |offset: f32, change: f32| -> f32 { offset - change * CONFIG.pan_speed / (100.) };
        camera_offset.0 = f(camera_offset.0, dx);
        camera_offset.1 = f(camera_offset.1, dy);

        clamp_camera(model);
    }
}

pub fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    model.pressed = Some(button);
}

pub fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.pressed = None;
}
