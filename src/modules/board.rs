use std::{
    any::{Any, TypeId},
    fs::DirEntry,
};

use crate::modules::{bank::Bank, field::Buyable, player::Player};

use super::{
    constants,
    field::{self, board_fields::create_fields, general_field::General, street::Street, Field},
};
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    fields: Vec<dyn Field>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: create_fields(),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
