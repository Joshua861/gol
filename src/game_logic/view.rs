use super::*;
use crate::board::Board;
use crate::time;
use crate::utils::clear;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let cache = &model.cache;

    if cache.board_to_pixel_array.is_empty() {
        return;
    }

    time!("view", {
        let draw = app.draw();
        let board = &model.board;
        draw.background().color(CONFIG.void_color.to_srgb());

        draw_background(&draw, cache);

        time!("cells", {
            draw_cells(&draw, board, cache);
        });

        draw_highlight(&draw, model);

        if model.grid_lines {
            time!("grid lines", {
                draw_grid_lines(&draw, cache);
            });
        }

        if model.show_fps {
            draw_fps(&draw, model)
        }

        draw.to_frame(app, &frame).unwrap();
    });

    #[cfg(debug_assertions)]
    clear();
}

fn draw_background(draw: &Draw, cache: &Cache) {
    draw.rect()
        .x_y(
            cache.camera_offset.0 * cache.scale_factor - 0.5 * cache.tile_size,
            cache.camera_offset.1 * cache.scale_factor - 0.5 * cache.tile_size,
        )
        .w_h(
            cache.board_width as f32 * cache.tile_size,
            cache.board_height as f32 * cache.tile_size,
        )
        .color(CONFIG.background_color.to_srgb());
}

fn draw_fps(draw: &Draw, model: &Model) {
    let cache = &model.cache;

    draw.text(&format!("{:.1}", model.fps.avg()))
        .color(CONFIG.cell_color.to_srgb())
        .x_y(
            -cache.window_size.0 / 2. + 40.,
            cache.window_size.1 / 2. - 25.,
        )
        .font_size(24)
        .w_h(100., 100.);
}

fn draw_cells(draw: &Draw, board: &Board, cache: &Cache) {
    for (i, tile) in board.tiles.iter().enumerate() {
        if *tile {
            let (px, py) = board_to_pixel(i, cache);

            draw.rect()
                .x_y(px, py)
                .w_h(cache.tile_size, cache.tile_size)
                .color(CONFIG.cell_color.to_srgb());
        }
    }
}

fn draw_highlight(draw: &Draw, model: &Model) {
    let cache = &model.cache;
    let board = &model.board;
    let (x, y) = pixel_to_board(f32_to_vec2(model.mouse_pos), cache);

    if let Some(v) = board.get(x, y) {
        let (px, py) = board_xy_to_pixel((x, y), cache);
        if v {
            draw.rect()
                .x_y(px, py)
                .w_h(cache.tile_size, cache.tile_size)
                .color(CONFIG.cell_color_highlighted.to_srgb());
        } else {
            draw.rect()
                .x_y(px, py)
                .w_h(cache.tile_size, cache.tile_size)
                .color(CONFIG.background_color_highlighted.to_srgb());
        }
    }
}

fn draw_grid_lines(draw: &Draw, cache: &Cache) {
    let mut weight = CONFIG.grid_thickness;

    if CONFIG.scale_grid_with_zoom {
        weight *= cache.scale_factor;
    }

    let (x, y) = board_xy_to_pixel((0, 0), cache);
    let mut x = x - 0.5 * cache.tile_size;
    let y = y - 0.5 * cache.tile_size;

    for _ in 0..=cache.board_width {
        draw.line()
            .start(pt2(x, y))
            .end(pt2(x, y + cache.board_height as f32 * cache.tile_size))
            .weight(weight)
            .color(CONFIG.grid_color.to_srgb());

        x += cache.tile_size;
    }

    for i in 0..=cache.board_height {
        let (x, y) = board_xy_to_pixel((0, i), cache);
        let x = x - 0.5 * cache.tile_size;
        let y = y - 0.5 * cache.tile_size;

        draw.line()
            .start(pt2(x, y))
            .end(pt2(x + cache.board_width as f32 * cache.tile_size, y))
            .weight(weight)
            .color(CONFIG.grid_color.to_srgb());
    }
}
