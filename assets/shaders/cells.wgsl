struct Cache {
  scale_factor: f32,
  tile_size: f32,
  half_board_width: f32,
  half_board_height: f32,
  camera_offset: vec2<f32>,
  cell_color: vec4<f32>,
  background_color: vec4<f32>,
  void_color: vec4<f32>,
};

struct Grid {
  width: u32,
};

@group(0) @binding(0) var<storage, read> tiles: array<u32>;
@group(0) @binding(1) var<uniform> grid: Grid;
@group(0) @binding(2) var<uniform> cache: Cache;

fn pixel_to_board(pixel: vec2<f32>, cache: Cache) -> vec2<u32> {
    return vec2<u32>(
        u32(round(((pixel.x - cache.camera_offset[0] * cache.scale_factor) / cache.tile_size)
            + cache.half_board_width)),
        u32(round(((pixel.y - cache.camera_offset[1] * cache.scale_factor) / cache.tile_size)
            + cache.half_board_height))
    );
}

// dont do anything
@vertex
fn vs_main(@builtin(position) coord_in: vec4<f32>) -> @builtin(position) vec4<f32> {
  return coord_in;
}

@fragment
fn fs_main(@builtin(position) coord_in: vec4<f32>) -> @location(0) vec4<f32> {
  let board = pixel_to_board(coord_in.xy, cache);
  let i = board.y * grid.width + board.x;

  if i >= arrayLength(&tiles) {
    return cache.void_color;
  } else if tiles[i] == 1u {  
    return cache.cell_color;
  } else {
    return cache.background_color;
  }
}
