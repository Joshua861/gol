use crate::prelude::*;
use crate::time;
use crate::timing::clear_timers;
use crate::ui::draw_notifications;
use crate::ui::{draw_info, Window};

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

        if let Some(selection) = &model.selection {
            selection.render(&draw, cache);
        }

        time!("notifications", {
            draw_notifications(app, &draw, model);
        });

        draw_info(&draw, model);

        Window::new()
            .text(&model.keybinds)
            .open(model.show_keybinds)
            .build()
            .render(&draw, cache, model);

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
    {
        let mut x = x - 0.5 * ts;
        let y = y - 0.5 * ts;

        for i in 0..=cache.board_width {
            draw.line()
                .start(pt2(x, y))
                .end(pt2(x, y + cache.board_height as f32 * ts))
                .weight(if i % 10 == 0 { weight * 2. } else { weight })
                .color(CONFIG.grid_color.to_srgb());

            x += ts;
        }
    }

    {
        let x = x - 0.5 * ts;
        let mut y = y - 0.5 * ts;
        for i in 0..=cache.board_width {
            draw.line()
                .start(pt2(x, y))
                .end(pt2(x + cache.board_height as f32 * ts, y))
                .weight(if i % 10 == 0 { weight * 2. } else { weight })
                .color(CONFIG.grid_color.to_srgb());

            y += ts;
        }
    }
}
