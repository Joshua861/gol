use super::*;

pub fn pixel_to_board(pixel: Vec2, cache: &Cache) -> (usize, usize) {
    (
        (((pixel.x - cache.camera_offset.0) / cache.tile_size) + cache.half_board_width).round()
            as usize,
        (((pixel.y - cache.camera_offset.1) / cache.tile_size) + cache.half_board_height).round()
            as usize,
    )
}

pub fn board_xy_to_pixel(board: (usize, usize), cache: &Cache) -> (f32, f32) {
    let i = board.0 + board.1 * cache.board_width;
    cache.board_to_pixel_array[i]
}

pub fn board_to_pixel(i: usize, cache: &Cache) -> (f32, f32) {
    cache.board_to_pixel_array[i]
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
    model.cache.target_tile_size = model
        .cache
        .target_tile_size
        .clamp(CONFIG.tile_size / 2., 100.0);

    let f = |board_side: usize, value: &mut f32| {
        let clamp_offset = (board_side as f32 * model.cache.tile_size) / 2.;
        *value = value.clamp(-clamp_offset, clamp_offset);
    };

    f(model.board.width(), &mut model.cache.target_camera_offset.0);
    f(
        model.board.height(),
        &mut model.cache.target_camera_offset.1,
    );
}
