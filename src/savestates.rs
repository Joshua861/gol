use crate::{game::Board, utils::BASE_DIR};
use chrono::Utc;
use std::fs;

pub fn save(board: Board) {
    let id = Utc::now().to_rfc3339();
    let serialized = bincode::serialize(&board).unwrap();
    fs::create_dir_all(savestate_dir()).unwrap();
    fs::write(format!("{}/{}", savestate_dir(), id), serialized)
        .unwrap_or_else(|e| eprintln!("Failed to save board state: {}", e));
}

pub fn load(id: String) -> Board {
    let text = fs::read(format!("{}/{}", savestate_dir(), id)).unwrap();

    bincode::deserialize(&text).unwrap()
}

fn savestate_dir() -> String {
    BASE_DIR.to_string() + "/savestates"
}
