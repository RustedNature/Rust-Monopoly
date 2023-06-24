#![allow(unused)]
pub mod modules {
    pub mod bank;
    pub mod board;
    pub mod color;
    pub mod constants;
    pub mod field;
    pub mod game_manager;
    pub mod io_manager;
    pub mod player;
}

use modules::game_manager::GameManager;

use crate::modules::{field::board_fields, player::Player};
fn main() {
    let mut gm = GameManager::new();
    gm.setup_game();
}

#[cfg(test)]
mod tests;
