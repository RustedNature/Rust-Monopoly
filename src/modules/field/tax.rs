use crate::modules::{field::Buyable, player::Player};

use super::FieldType;
#[derive(Clone, PartialEq, Eq)]

pub struct Tax {
    name: String,
    tax_to_pay: i32,
}

impl Tax {
    pub fn new(name: String, tax_to_pay: i32) -> Tax {
        Tax { name, tax_to_pay }
    }
    //TODO
}
impl FieldType for Tax {
    fn get_name(&self) -> &str {
        &self.name
    }
}
