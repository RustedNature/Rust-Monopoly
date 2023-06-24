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

    pub fn get_name_of_field(&self, field_index: u32) -> &str {
        let mut field = &self.fields[field_index as usize];
        let mut processed_field = field.process_field();
        processed_field.get_name()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
