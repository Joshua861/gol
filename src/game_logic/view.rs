use super::*;
use crate::game::Board;
use crate::time;
use crate::timing::{clear_timers, get_timers};
use crate::utils::VERSION;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let cache = &model.cache;

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

        if model.show_info {
            draw_info(&draw, model)
        }

        clear_timers();

        draw.to_frame(app, &frame).unwrap();
    });
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

fn draw_info(draw: &Draw, model: &Model) {
    let cache = &model.cache;

    let mut text = format!("{:.2} fps", model.fps.avg());

    if model.paused {
        text = format!("{} (paused)", text);
    }

    text = format!(
        "{}\ngrid: ({} x {})\nwindow: ({} x {})\nrulestring: {}\nv{}\ncamera offset: ({:.1} x {:.1})\nzoom: {:.2}",
        text,
        model.cache.board_width,
        model.cache.board_height,
        model.cache.window_size.0,
        model.cache.window_size.1,
        model.rulestring,
        VERSION,
        model.cache.camera_offset.0,
        model.cache.camera_offset.1,
        model.cache.scale_factor
    );

    if model.symmetry {
        text = format!("{}\nSymmetry on", text);
    }

    if model.grid_lines {
        text = format!("{}\nGrid on", text);
    }

    #[cfg(debug_assertions)]
    {
        text += "\n";
        for timer in get_timers() {
            text = format!("{}\n{}", text, timer);
        }
    }

    draw.text(&text)
        .color(CONFIG.cell_color.to_srgb())
        .x_y(
            -cache.window_size.0 / 2. + 515.,
            cache.window_size.1 / 2. - 60.,
        )
        .font_size(24)
        .font(model.font.clone())
        .left_justify()
        .align_text_top()
        .w_h(1000., 100.);
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
    let ts = cache.tile_size;

    if CONFIG.scale_grid_with_zoom {
        weight *= cache.scale_factor;
    }

    let (x, y) = board_xy_to_pixel((0, 0), cache);
    let mut x = x - 0.5 * ts;
    let y = y - 0.5 * ts;

    for _ in 0..=cache.board_width {
        draw.line()
            .start(pt2(x, y))
            .end(pt2(x, y + cache.board_height as f32 * ts))
            .weight(weight)
            .color(CONFIG.grid_color.to_srgb());

        x += ts;
    }

    for i in 0..=cache.board_height {
        let (x, y) = board_xy_to_pixel((0, i), cache);
        let x = x - 0.5 * ts;
        let y = y - 0.5 * ts;

        draw.line()
            .start(pt2(x, y))
            .end(pt2(x + cache.board_width as f32 * ts, y))
            .weight(weight)
            .color(CONFIG.grid_color.to_srgb());
    }
}
