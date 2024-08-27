use config::CONFIG;
use game::Game;
use nannou::prelude::*;
use utils::clear;

mod config;
mod game;
mod utils;

struct Model {
    game: Game,
    paused: bool,
    pressed: Option<MouseButton>,
    last_mouse_pos: Option<(usize, usize)>,
    cache: CoordinateCache,
    mouse_pos: (f32, f32),
    grid_lines: bool,
}

struct CoordinateCache {
    half_board_width: f32,
    half_board_height: f32,
    tile_size: f32,
    board_width: usize,
    board_height: usize,
    scale_factor: f32,
}

impl CoordinateCache {
    fn new(board_size: (usize, usize), tile_size: f32) -> Self {
        Self {
            half_board_width: board_size.0 as f32 / 2.,
            half_board_height: board_size.1 as f32 / 2.,
            tile_size,
            board_width: board_size.0,
            board_height: board_size.1,
            scale_factor: 1.,
        }
    }

    fn update(&mut self, board_size: (usize, usize), tile_size: f32) {
        self.half_board_width = board_size.0 as f32 / 2.;
        self.half_board_height = board_size.1 as f32 / 2.;
        self.tile_size = tile_size;
        self.board_width = board_size.0;
        self.board_height = board_size.1;
        self.scale_factor = tile_size / CONFIG.tile_size;
    }
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
        .mouse_moved(mouse_moved)
        .mouse_wheel(mouse_wheel) // Add scroll event handler
        .build()
        .unwrap();

    let initial_tile_size = CONFIG.tile_size;
    let board_size = (1, 1);

    Model {
        game: Game::new(1, 1),
        paused: false,
        pressed: None,
        last_mouse_pos: None,
        cache: CoordinateCache::new(board_size, initial_tile_size), // Initialize cache
        mouse_pos: (0.0, 0.0),
        grid_lines: false,
    }
}

fn mouse_wheel(_app: &App, model: &mut Model, delta: MouseScrollDelta, _phase: TouchPhase) {
    match delta {
        MouseScrollDelta::LineDelta(_, y) => {
            let new_tile_size =
                (model.cache.tile_size + y * CONFIG.zoom_speed).clamp(CONFIG.tile_size, 100.0);

            model.cache.update(model.game.wh(), new_tile_size);
        }
        MouseScrollDelta::PixelDelta(_) => {}
    }
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    model.mouse_pos = (pos.x, pos.y);
}

fn window_resized(_app: &App, model: &mut Model, rect: Vec2) {
    time!("window_resize", {
        let width = (rect.x / CONFIG.tile_size).ceil() as usize;
        let height = (rect.y / CONFIG.tile_size).ceil() as usize;
        model.game.set_wh(width, height);

        model.cache.update((width, height), CONFIG.tile_size);
    });
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.paused = !model.paused,
        Key::C => model.game.clear(),
        Key::G => model.grid_lines = !model.grid_lines,
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
    if !model.paused {
        time!("advance", { model.game.advance() });
    }
    if let Some(button) = model.pressed {
        let mut set = |to: bool| {
            let pos = app.mouse.position();
            let (x, y) = pixel_to_game(pos, &model.cache);

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

        model.last_mouse_pos = Some(pixel_to_game(app.mouse.position(), &model.cache));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    time!("view", {
        let draw = app.draw();
        let game = &model.game;
        draw.background().color(CONFIG.background_color.to_srgb());

        time!("rects", {
            for (i, tile) in game.tiles.iter().enumerate() {
                if *tile {
                    let x = i % model.cache.board_width;
                    let y = i / model.cache.board_width;
                    let (px, py) = game_to_pixel((x, y), &model.cache);
                    draw.rect()
                        .x_y(px, py)
                        .w_h(model.cache.tile_size, model.cache.tile_size)
                        .color(CONFIG.cell_color.to_srgb());
                }
            }
        });

        time!("cursor highlight", {
            let (x, y) = pixel_to_game(f32_to_vec2(model.mouse_pos), &model.cache);

            if let Some(v) = game.get(x, y) {
                let (px, py) = game_to_pixel((x, y), &model.cache);
                if v {
                    draw.rect()
                        .x_y(px, py)
                        .w_h(model.cache.tile_size, model.cache.tile_size)
                        .color(CONFIG.cell_color_highlighted.to_srgb());
                } else {
                    draw.rect()
                        .x_y(px, py)
                        .w_h(model.cache.tile_size, model.cache.tile_size)
                        .color(CONFIG.background_color_highlighted.to_srgb());
                }
            }
        });

        if model.grid_lines {
            time!("grid lines", {
                let mut weight = CONFIG.grid_thickness;

                if CONFIG.scale_grid_with_zoom {
                    weight *= model.cache.scale_factor;
                }

                for i in 0..model.cache.board_width {
                    let (x, _) = game_to_pixel((i, 0), &model.cache);
                    let (_, y_end) = game_to_pixel((i, model.cache.board_height - 1), &model.cache);
                    let x = x + 0.5 * model.cache.tile_size;

                    draw.line()
                        .start(pt2(
                            x,
                            -model.cache.half_board_height * model.cache.tile_size,
                        ))
                        .end(pt2(x, y_end))
                        .weight(weight)
                        .color(CONFIG.grid_color.to_srgb());
                }

                for i in 0..model.cache.board_height {
                    let (_, y) = game_to_pixel((0, i), &model.cache);
                    let (x_end, _) = game_to_pixel((model.cache.board_width - 1, i), &model.cache);
                    let y = y + 0.5 * model.cache.tile_size;

                    draw.line()
                        .start(pt2(
                            -model.cache.half_board_width * model.cache.tile_size,
                            y,
                        ))
                        .end(pt2(x_end, y))
                        .weight(weight)
                        .color(CONFIG.grid_color.to_srgb());
                }
            });
        }

        draw.to_frame(app, &frame).unwrap();
    });

    #[cfg(debug_assertions)]
    clear();
}

fn pixel_to_game(pixel: Vec2, cache: &CoordinateCache) -> (usize, usize) {
    (
        ((pixel.x / cache.tile_size) + cache.half_board_width).round() as usize,
        ((pixel.y / cache.tile_size) + cache.half_board_height).round() as usize,
    )
}

fn game_to_pixel(board: (usize, usize), cache: &CoordinateCache) -> (f32, f32) {
    (
        (board.0 as f32 - cache.half_board_width) * cache.tile_size,
        (board.1 as f32 - cache.half_board_height) * cache.tile_size,
    )
}

fn f32_to_vec2(f: (f32, f32)) -> Vec2 {
    Vec2::new(f.0, f.1)
}
