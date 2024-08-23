use grid::Grid;

use crate::{chance, config::CONFIG};

#[derive(Clone)]
pub struct Game {
    tiles: Grid<bool>,
    frame_count: usize,
}

impl Game {
    pub fn new() -> Self {
        let tiles = Grid::from_vec(
            vec![false; CONFIG.board_width * CONFIG.board_height]
                .iter()
                .map(|_| {
                    if chance!(1 in 3) {
                        return true;
                    }

                    false
                })
                .collect::<Vec<bool>>(),
            CONFIG.board_width,
        );

        Self {
            tiles,
            frame_count: 0,
        }
    }
    pub fn advance(&mut self) {
        let mut next = self.clone();

        for i in 0..(CONFIG.board_width * CONFIG.board_height) {
            let (x, y) = Game::i_to_xy(i);

            let count = self.get_neighbors(x, y).iter().filter(|v| **v).count();

            if !(2..=3).contains(&count) {
                next.set(x, y, false);
            } else if count == 3 {
                next.set(x, y, true);
            }
        }

        next.frame_count += 1;

        *self = next;
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
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        *self.tiles.get_mut(y, x).expect("Failed to set tile.") = value;
    }
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<bool> {
        let mut neighbors = Vec::new();

        [
            [1, 1],
            [1, 0],
            [0, 1],
            [-1, -1],
            [-1, 0],
            [0, -1],
            [1, -1],
            [-1, 1],
        ]
        .iter()
        .for_each(|[x_offset, y_offset]| {
            neighbors.push(self.get_or_empty(x as isize + x_offset, y as isize + y_offset))
        });

        neighbors
    }
    pub fn i_to_xy(i: usize) -> (usize, usize) {
        (i % CONFIG.board_width, i / CONFIG.board_width)
    }
}
