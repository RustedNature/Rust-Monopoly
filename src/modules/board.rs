use crate::modules::{bank::Bank, player::Player};

use super::field::{general_field::General, street::Street, Field};
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
            fields: vec![],
        }
    }
}
