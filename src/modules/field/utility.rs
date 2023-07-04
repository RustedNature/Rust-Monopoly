use crate::modules::{field::Buyable, player::Player};

use super::FieldType;
#[derive(Clone, PartialEq, Eq)]

pub struct Utility {
    name: String,
    buy_cost: i32,
    has_owner: bool,
    owner: Option<Player>,
    color: Buyable,
}

impl Utility {
    pub fn new(name: String) -> Utility {
        Utility {
            name,
            buy_cost: 150,
            has_owner: false,
            owner: None,
            color: Buyable::Utility,
        }
    }
    //TODO
}

impl FieldType for Utility {
    fn get_name(&self) -> &str {
        &self.name
    }
}
