use crate::modules::{bank::Bank, color::Color, player::Player};

use super::{
    constants,
    field::{board_fields::create_fields, Field},
};
struct Board {
    bank: Bank,
    players: Vec<Player>,
    fields: Vec<Field>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bank: Bank::new(),
            players: Vec::new(),
            fields: create_fields(),
        }
    }
}
