use crate::modules::{bank::Bank, player::Player};
struct Board {
    bank: Bank,
    players: Vec<Player>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bank: Bank::new(),
            players: Vec::new(),
        }
    }
}
