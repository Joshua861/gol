use config::CONFIG;
use game::Game;
use nannou::prelude::*;

mod config;
mod game;
mod utils;

struct Model {
    game: Game,
    paused: bool,
    pressed: Option<MouseButton>,
    last_mouse_pos: Option<(usize, usize)>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .resized(window_resized)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        game: Game::new(1, 1),
        paused: false,
        pressed: None,
        last_mouse_pos: None,
    }
}

fn window_resized(_app: &App, model: &mut Model, rect: Vec2) {
    time!("window_resize", {
        model.game.set_wh(
            (rect.x / CONFIG.tile_size).ceil() as usize,
            (rect.y / CONFIG.tile_size).ceil() as usize,
        );
    });
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.paused = !model.paused,
        Key::C => model.game.clear(),
        _ => (),
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    model.pressed = Some(button);
}

fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.pressed = None;
    model.last_mouse_pos = None;
}

fn update(app: &App, model: &mut Model, _update: Update) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    if !model.paused {
        time!("advance", { model.game.advance() });
    }
    if let Some(button) = model.pressed {
        let mut set = |to: bool| {
            let pos = app.mouse.position();
            let (x, y) = pixel_to_game(pos, model.game.wh());

            if let Some((px, py)) = model.last_mouse_pos {
                model.game.draw_line(x, y, px, py, to);
            } else {
                model.game.set(x, y, to);
            }
        };

        match button {
            MouseButton::Left => set(true),
            MouseButton::Right => set(false),
            _ => (),
        }

        model.last_mouse_pos = Some(pixel_to_game(app.mouse.position(), model.game.wh()));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    time!("view", {
        let draw = app.draw();
        let game = &model.game;

        draw.background().color(Srgb::new(0.1, 0.1, 0.1));

        time!("rects", {
            for (i, tile) in game.tiles.iter().enumerate() {
                if *tile {
                    let x = i % game.width();
                    let y = i / game.width();

                    let (px, py) = board_to_game((x, y), game.wh());

                    draw.rect()
                        .x_y(px, py)
                        .w_h(CONFIG.tile_size, CONFIG.tile_size)
                        .color(Srgb::new(0.9, 0.9, 0.9));
                }
            }
        });

        draw.to_frame(app, &frame).unwrap();
    });
}

fn pixel_to_game(pixel: Vec2, board_size: (usize, usize)) -> (usize, usize) {
    (
        ((pixel.x / CONFIG.tile_size) + board_size.0 as f32 / 2.).round() as usize,
        ((pixel.y / CONFIG.tile_size) + board_size.1 as f32 / 2.).round() as usize,
    )
}

fn board_to_game(board: (usize, usize), board_size: (usize, usize)) -> (f32, f32) {
    (
        (board.0 as f32 - board_size.0 as f32 / 2.) * CONFIG.tile_size,
        (board.1 as f32 - board_size.1 as f32 / 2.) * CONFIG.tile_size,
    )
}
