use crate::prelude::*;

pub fn pixel_to_board(pixel: Vec2, cache: &Cache) -> (usize, usize) {
    (
        (((pixel.x - cache.camera_offset.0 * cache.scale_factor) / cache.tile_size)
            + cache.half_board_width)
            .round() as usize,
        (((pixel.y - cache.camera_offset.1 * cache.scale_factor) / cache.tile_size)
            + cache.half_board_height)
            .round() as usize,
    )
}

pub fn board_xy_to_pixel(board: (usize, usize), cache: &Cache) -> (f32, f32) {
    let (x, y) = board;

    (
        (x as f32 - cache.half_board_width) * cache.tile_size
            + (cache.camera_offset.0 * cache.scale_factor),
        (y as f32 - cache.half_board_height) * cache.tile_size
            + (cache.camera_offset.1 * cache.scale_factor),
    )
}

pub fn board_to_pixel(i: usize, cache: &Cache) -> (f32, f32) {
    board_xy_to_pixel(i_to_xy(cache.board_width, i), cache)
}

pub fn f32_to_vec2(f: (f32, f32)) -> Vec2 {
    Vec2::new(f.0, f.1)
}

pub fn vec2_to_f32(v: Vec2) -> (f32, f32) {
    (v.x, v.y)
}

pub fn i_to_xy(width: usize, i: usize) -> (usize, usize) {
    (i % width, i / width)
}

pub fn clamp_camera(model: &mut Model) {
    if CONFIG.autosize_board {
        model.cache.target_tile_size = model
            .cache
            .target_tile_size
            .clamp(CONFIG.tile_size / 2., 100.0);
    } else {
        model.cache.target_tile_size = model.cache.target_tile_size.clamp(1., 100.0);
    }

    let f = |board_side: usize, value: &mut f32| {
        let clamp_offset = (board_side as f32 * CONFIG.tile_size) / 2.;
        *value = value.clamp(-clamp_offset, clamp_offset);
    };

    f(model.board.width(), &mut model.cache.target_camera_offset.0);
    f(
        model.board.height(),
        &mut model.cache.target_camera_offset.1,
    );
}

pub fn print_grid(grid: Grid<bool>) {
    let mut tiles = grid.clone();
    tiles.flip_rows();
    tiles.flip_cols();

    for r in tiles.iter_rows() {
        r.for_each(|v| print!("{}", if *v { "#" } else { " " }));
        println!();
    }
}
