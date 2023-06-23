use crate::modules::{bank::Bank, color::Color, player::Player};

use super::{
    constants,
    field::{board_fields::create_fields, Field},
};
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    fields: Vec<Field>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: create_fields(),
        }
    }
}
