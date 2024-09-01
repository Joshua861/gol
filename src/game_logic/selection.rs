use crate::prelude::*;
use nalgebra::{Matrix2, Vector2};

#[derive(Clone)]
pub struct Selection {
    pub start: VecU2,
    pub end: VecU2,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq)]
pub enum Rotation {
    CW,
    CCW,
}

impl Selection {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            start: VecU2::new(x, y),
            end: VecU2::new(x, y),
        }
    }
    pub fn render(&self, draw: &Draw, cache: &Cache) {
        outline(draw, cache, self.start, self.end);
    }
    pub fn width(&self) -> usize {
        self.start.x.abs_diff(self.end.x)
    }
    pub fn height(&self) -> usize {
        self.start.y.abs_diff(self.end.y)
    }
    pub fn wh(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
    pub fn get_inner_tiles(&self, model: &Model) -> Grid<bool> {
        let (w, h) = self.wh();
        let (w, h) = (w + 1, h + 1);
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);
        let board = &model.board;

        let mut grid = Grid::new(h, w);

        for dx in 0..w {
            for dy in 0..h {
                let (x, y) = (min_x + dx, min_y + dy);

                if let Some(tile) = grid.get_mut(dy, dx) {
                    *tile = board.get_or_empty(x as isize, y as isize);
                } else {
                    panic!("Attempted to access out of bounds tile at ({}, {})", dx, dy);
                }
            }
        }

        grid
    }
    pub fn copy(&self, model: &mut Model) {
        model.clipboard = Some(self.get_inner_tiles(model));
    }
    pub fn paste(model: &mut Model) {
        if let Some(clipboard) = &model.clipboard {
            let (x, y) = pixel_to_board(model.mouse_pos.into(), &model.cache);
            let (w, h) = (clipboard.cols(), clipboard.rows());
            model.board.set_area(VecU2::new(x, y), clipboard);
            model.selection = Some(Selection {
                start: VecU2::new(x, y),
                end: VecU2::new(x + w - 1, y + h - 1),
            });
        }
    }
    pub fn clear(&self, model: &mut Model) {
        let (w, h) = self.wh();
        let (w, h) = (w + 1, h + 1);
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);

        for dx in 0..w {
            for dy in 0..h {
                let (x, y) = (min_x + dx, min_y + dy);

                model.board.set(x, y, false);
            }
        }
    }
    pub fn rotate(&self, model: &mut Model, rotation: Rotation) {
        let (w, h) = self.wh();
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);

        let center_x = min_x as isize + w as isize / 2;
        let center_y = min_y as isize + h as isize / 2;

        let matrix = match rotation {
            Rotation::CW => Matrix2::new(0, -1, 1, 0),
            Rotation::CCW => Matrix2::new(0, 1, -1, 0),
        };

        let original_grid = self.get_inner_tiles(model);

        for dx in 0..=w {
            for dy in 0..=h {
                model.board.set(min_x + dx, min_y + dy, false);
            }
        }

        let mut new_min_x = isize::MAX;
        let mut new_max_x = isize::MIN;
        let mut new_min_y = isize::MAX;
        let mut new_max_y = isize::MIN;

        for dx in 0..=w {
            for dy in 0..=h {
                if let Some(&tile) = original_grid.get(dy, dx) {
                    if !tile {
                        continue;
                    }

                    let (x, y) = (min_x + dx, min_y + dy);

                    let vector = Vector2::new(x as isize - center_x, y as isize - center_y);

                    let transformed_vector = matrix * vector;

                    let new_x = (transformed_vector.x + center_x) as usize;
                    let new_y = (transformed_vector.y + center_y) as usize;

                    model.board.set(new_x, new_y, tile);

                    new_min_x = new_min_x.min(new_x as isize);
                    new_max_x = new_max_x.max(new_x as isize);
                    new_min_y = new_min_y.min(new_y as isize);
                    new_max_y = new_max_y.max(new_y as isize);
                }
            }
        }

        let new_selection = Selection {
            start: VecU2::new(new_min_x as usize, new_min_y as usize),
            end: VecU2::new(new_max_x as usize, new_max_y as usize),
        };

        model.selection = Some(new_selection);
    }
    pub fn translate(&self, model: &mut Model, dx: isize, dy: isize) {
        let (w, h) = self.wh();
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);

        let original_grid = self.get_inner_tiles(model);

        for dx in 0..=w {
            for dy in 0..=h {
                model.board.set(min_x + dx, min_y + dy, false);
            }
        }

        for dx_offset in 0..=w {
            for dy_offset in 0..=h {
                if let Some(&tile) = original_grid.get(dy_offset, dx_offset) {
                    if tile {
                        let new_x = (min_x as isize + dx_offset as isize + dx) as usize;
                        let new_y = (min_y as isize + dy_offset as isize + dy) as usize;
                        model.board.set(new_x, new_y, true);
                    }
                }
            }
        }

        let new_selection = Selection {
            start: VecU2::new(
                (self.start.x as isize + dx) as usize,
                (self.start.y as isize + dy) as usize,
            ),
            end: VecU2::new(
                (self.end.x as isize + dx) as usize,
                (self.end.y as isize + dy) as usize,
            ),
        };

        model.selection = Some(new_selection);
    }
}

pub fn outline(draw: &Draw, cache: &Cache, start: VecU2, end: VecU2) {
    let (sx, sy) = board_xy_to_pixel(start.as_tuple(), cache);
    let (ex, ey) = board_xy_to_pixel(end.as_tuple(), cache);
    let rect = Rect::from_corners(Vec2::new(sx, sy), Vec2::new(ex, ey));

    let offset = 0.5 * cache.tile_size;

    let mut start = rect.bottom_left();
    start.x -= offset;
    start.y -= offset;
    let mut end = rect.top_right();
    end.x += offset;
    end.y += offset;

    for [[x, y], [ex, ey]] in [
        [[start.x, start.y], [start.x, end.y]],
        [[start.x, start.y], [end.x, start.y]],
        [[end.x, start.y], [end.x, end.y]],
        [[end.x, end.y], [start.x, end.y]],
    ] {
        draw.line()
            .color(CONFIG.selection_color.to_srgb())
            .weight(CONFIG.selection_thickness)
            .start(Vec2::new(x, y))
            .end(Vec2::new(ex, ey));
    }
}
