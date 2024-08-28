use crate::{game::Board, utils::BASE_DIR};
use chrono::{Datelike, Local, Timelike};
use grid::Grid;
use serde::de;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs;

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut board = self.clone();

        for _ in 0..2 {
            for c in self.tiles.iter_cols() {
                if c.filter(|v| **v).collect::<Vec<&bool>>().is_empty() {
                    board.tiles.remove_col(0);
                } else {
                    break;
                }
            }

            for r in self.tiles.iter_rows() {
                if r.filter(|v| **v).collect::<Vec<&bool>>().is_empty() {
                    board.tiles.remove_row(0);
                } else {
                    break;
                }
            }

            board.tiles.rotate_half();
        }

        let mut rle_tiles: Vec<u8> = Vec::new();

        let mut current_value = board.tiles.get(0, 0).unwrap_or(&false);
        let mut current_count = 0;

        for tile in board.tiles.iter() {
            if *tile == *current_value {
                current_count += 1;
            } else {
                rle_tiles.push(current_count);
                rle_tiles.push(if *current_value { 1 } else { 0 });
                current_value = tile;
                current_count = 1;
            }
        }

        rle_tiles.push(current_count);
        rle_tiles.push(if *current_value { 1 } else { 0 });

        let width = board.width();
        let height = board.height();

        let mut state = serializer.serialize_struct("Board", 4)?;
        state.serialize_field("width", &width)?;
        state.serialize_field("height", &height)?;
        state.serialize_field("rle_tiles", &rle_tiles)?;
        state.serialize_field("run_count", &self.run_count)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BoardData {
            width: usize,
            height: usize,
            rle_tiles: Vec<u8>,
            run_count: usize,
        }

        let data = BoardData::deserialize(deserializer)?;

        let total_cells = data.width * data.height;
        let mut bool_tiles: Vec<bool> = Vec::with_capacity(total_cells);

        let mut i = 0;
        while i < data.rle_tiles.len() {
            if i + 1 >= data.rle_tiles.len() {
                return Err(de::Error::custom(
                    "Invalid RLE data: Missing value for run length",
                ));
            }

            let count = data.rle_tiles[i] as usize;
            let value = data.rle_tiles[i + 1] != 0;

            for _ in 0..count {
                bool_tiles.push(value);
            }

            i += 2;
        }

        if bool_tiles.len() != total_cells {
            return Err(de::Error::custom(
                "Decoded tile count does not match expected width * height",
            ));
        }

        let grid = Grid::from_vec(bool_tiles, data.width);

        Ok(Board {
            tiles: grid,
            run_count: data.run_count,
        })
    }
}

pub fn save(board: Board) {
    let time = Local::now();
    let id = format!(
        "{}-{}-{} {}:{}",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute()
    );
    let serialized = bincode::serialize(&board).unwrap();
    fs::create_dir_all(savestate_dir()).unwrap();
    fs::write(format!("{}/{}.gol", savestate_dir(), id), serialized)
        .unwrap_or_else(|e| eprintln!("Failed to save board state: {}", e));
}

pub fn load(id: String) -> Board {
    let text = fs::read(format!("{}/{}.gol", savestate_dir(), id)).unwrap();

    bincode::deserialize(&text).unwrap()
}

fn savestate_dir() -> String {
    BASE_DIR.to_string() + "/savestates"
}
