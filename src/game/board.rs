use crate::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

use crate::{config::CONFIG, game_logic::i_to_xy};

#[derive(Clone, Debug)]
pub struct Board {
    pub tiles: Grid<bool>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = Grid::from_vec(vec![false; width * height], width);

        Self { tiles }
    }
    pub fn advance(&mut self) {
        if !CONFIG.parallel_board_processing {
            let width = self.width();
            let rule = &CONFIG.rule;

            let mut next_tiles = self.clone();

            let mut active_tiles = self
                .tiles
                .iter()
                .enumerate()
                .filter(|(_, v)| **v)
                .map(|(i, _)| i_to_xy(width, i))
                .collect::<HashSet<_>>();

            active_tiles.clone().iter().for_each(|&(x, y)| {
                [
                    [1, 1],
                    [1, 0],
                    [0, 1],
                    [-1, 0],
                    [0, -1],
                    [-1, -1],
                    [-1, 1],
                    [1, -1],
                ]
                .iter()
                .for_each(|[dx, dy]| {
                    active_tiles.insert(((dx + x as isize) as usize, (dy + y as isize) as usize));
                })
            });

            for (x, y) in active_tiles.into_iter() {
                let count = self.count_neighbors(x, y);
                let cell = self.get(x, y).unwrap_or(false);
                next_tiles.set(
                    x,
                    y,
                    (!cell && rule.born(count)) || (cell && rule.survive(count)),
                );
            }

            *self = next_tiles;
        } else {
            let width = self.width();
            let height = self.height();
            let rule = &CONFIG.rule;

            let mut next_tiles = vec![false; width * height];

            next_tiles.par_iter_mut().enumerate().for_each(|(i, tile)| {
                let (x, y) = self.i_to_xy(i);
                let count = self.count_neighbors(x, y);
                let cell = self.get(x, y).unwrap_or(false);
                *tile = (!cell && rule.born(count)) || (cell && rule.survive(count));
            });

            self.tiles = Grid::from_vec(next_tiles, width);
        }
    }
    pub fn width(&self) -> usize {
        self.tiles.cols()
    }
    pub fn height(&self) -> usize {
        self.tiles.rows()
    }
    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.tiles.get(y, x).cloned()
    }
    pub fn get_or_empty(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        self.tiles.get(y, x).cloned().unwrap_or(false)
    }
    pub fn set_wh(&mut self, w: usize, h: usize) {
        let mut new_game = Board::new(w, h);
        let x_offset = (w as isize - self.width() as isize) / 2;
        let y_offset = (h as isize - self.height() as isize) / 2;

        for (i, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.i_to_xy(i);

            new_game.try_set(
                (x as isize + x_offset) as usize,
                (y as isize + y_offset) as usize,
                *tile,
            );
        }

        *self = new_game
    }
    pub fn wh(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if let Some(tile) = self.tiles.get_mut(y, x) {
            *tile = value;
        }
    }
    pub fn try_set(&mut self, x: usize, y: usize, value: bool) -> Option<()> {
        *self.tiles.get_mut(y, x)? = value;
        Some(())
    }
    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.width() as i32 && ny >= 0 && ny < self.height() as i32 {
                    count += self.get_or_empty(nx as isize, ny as isize) as u8;
                }
            }
        }
        count
    }
    pub fn set_area(&mut self, pos: VecU2, tiles: &Grid<bool>) {
        let (dx, dy) = pos.as_tuple();
        let (w, h) = (tiles.cols(), tiles.rows());

        for x in 0..w {
            for y in 0..h {
                self.set(
                    x + dx,
                    y + dy,
                    *tiles.get(y, x).unwrap_or_else(|| {
                        notify_error("Failed to get tile (set area).");
                        &false
                    }),
                );
            }
        }
    }
    pub fn i_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width(), i / self.width())
    }
    pub fn clear(&mut self) {
        *self = Self::new(self.width(), self.height());
    }
    pub fn crop(&mut self) {
        for _ in 0..2 {
            while self.width() > 0 {
                let mut first_col = self.tiles.iter_col(0);
                if first_col.all(|v| !v) {
                    self.tiles.remove_col(0);
                } else {
                    break;
                }
            }

            while self.height() > 0 {
                let mut first_row = self.tiles.iter_row(0);
                if first_row.all(|v| !v) {
                    self.tiles.remove_row(0);
                } else {
                    break;
                }
            }

            self.tiles.rotate_half();
        }
    }
    pub fn draw_line(
        &mut self,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        to: bool,
    ) {
        let mut coords: HashSet<(usize, usize)> = HashSet::new();

        let mut x = start_x as isize;
        let mut y = start_y as isize;

        let dx = (end_x as isize - start_x as isize).abs();
        let dy = -(end_y as isize - start_y as isize).abs();
        let sx = if start_x < end_x { 1 } else { -1 };
        let sy = if start_y < end_y { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            coords.insert((x as usize, y as usize));
            if x == end_x as isize && y == end_y as isize {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                if x == end_x as isize {
                    break;
                }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == end_y as isize {
                    break;
                }
                err += dx;
                y += sy;
            }
        }

        coords.insert((end_x, end_y));

        coords.iter().for_each(|(x, y)| {
            self.set(*x, *y, to);
        });
    }
    pub fn print(&self) {
        print_grid(self.tiles.clone());
    }
}
