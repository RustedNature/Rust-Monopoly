use crate::modules::{color::Color, player::Player};

use super::FieldType;
#[derive(Clone, PartialEq, Eq)]

pub struct Utility {
    name: String,
    buy_cost: i32,
    has_owner: bool,
    owner: Option<Player>,
    color: Color,
}

impl Utility {
    pub fn new(name: String) -> Utility {
        Utility {
            name,
            buy_cost: 150,
            has_owner: false,
            owner: None,
            color: Color::Utility,
        }
    }
    //TODO
}

impl FieldType for Utility {
    fn get_name(&self) -> &str {
        &self.name
    }
}
