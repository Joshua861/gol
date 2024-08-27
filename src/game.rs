use grid::Grid;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Game {
    pub tiles: Grid<bool>,
    pub run_count: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = Grid::from_vec(vec![false; width * height], width);

        Self {
            tiles,
            run_count: 0,
        }
    }
    pub fn advance(&mut self) {
        let width = self.width();
        let height = self.height();

        let mut next_tiles = vec![false; width * height];

        next_tiles.par_iter_mut().enumerate().for_each(|(i, tile)| {
            let (x, y) = self.i_to_xy(i);
            let count = self.count_neighbors(x, y);
            let cell = self.get(x, y).unwrap_or(false);
            *tile = (count == 3) || (cell && count == 2);
        });

        self.tiles = Grid::from_vec(next_tiles, width);
        self.run_count += 1;
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
    // pub fn par_for_each(&self, f: impl Fn(usize, usize, bool) + Sync) {
    //     vec![false; self.width() * self.height()]
    //         .par_iter()
    //         .enumerate()
    //         .for_each(|(i, _)| {
    //             let (x, y) = self.i_to_xy(i);
    //
    //             f(x, y, self.get(x, y).unwrap())
    //         })
    // }
    #[allow(clippy::comparison_chain)]
    pub fn set_wh(&mut self, w: usize, h: usize) {
        let mut new_game = Game::new(w, h);

        for (i, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.i_to_xy(i);

            new_game.try_set(x, y, *tile);
        }

        *self = new_game
    }
    pub fn wh(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if let Some(tile) = self.tiles.get_mut(y, x) {
            *tile = value;
        } else {
            eprintln!(
                "Failed to set tile {}/{}.\n Board {{\n    width: {},\n    height: {},\n}}.",
                x,
                y,
                self.width(),
                self.height()
            );
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
    pub fn i_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width(), i / self.width())
    }
    pub fn clear(&mut self) {
        *self = Self::new(self.width(), self.height());
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
}
