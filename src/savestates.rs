use crate::{game::Board, utils::BASE_DIR};
use bitvec::prelude::*;
use chrono::{Datelike, Local, Timelike};
use grid::Grid;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs;

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut board = self.clone();
        board.crop();
        let width = board.width();

        let mut bv: BitVec<u8, Lsb0> = BitVec::new();
        board.tiles.iter().for_each(|v| bv.push(*v));

        let mut state = serializer.serialize_struct("Board", 4)?;
        state.serialize_field("width", &width)?;
        state.serialize_field("tiles", &bv)?;
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
            tiles: BitVec<u8, Lsb0>,
        }

        let mut data = BoardData::deserialize(deserializer)?;

        let tiles = (0..data.tiles.len())
            .map(|_| data.tiles.pop().unwrap())
            .rev()
            .collect::<Vec<bool>>();
        let mut grid = Grid::from_vec(tiles, data.width);
        grid.rotate_half();

        Ok(Board { tiles: grid })
    }
}

pub fn save_board(board: Board) {
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

pub fn load_savestate(id: String) -> Board {
    let text = fs::read(format!("{}/{}.gol", savestate_dir(), id)).unwrap();

    bincode::deserialize(&text).unwrap()
}

fn savestate_dir() -> String {
    BASE_DIR.to_string() + "/savestates"
}
