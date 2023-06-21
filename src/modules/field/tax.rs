use crate::modules::{color::Color, player::Player};

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
