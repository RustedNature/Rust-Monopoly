use crate::modules::{color::Color, player::Player};

use super::{
    rental_priceses::{self, RentalList, RentalPriceFor},
    FieldType,
};
#[derive(Clone, PartialEq, Eq)]
pub struct Street {
    name: String,
    has_owner: bool,
    buy_price: u32,
    house_counter: u8,
    is_mortaged: bool,
    owner: Option<Player>,
    rental_priceses: RentalList,
    price_per_house: u32,
    color: Color,
}

impl Street {
    pub fn new(
        name: String,
        color: Color,
        buy_price: u32,
        price_per_house: u32,
        rental_priceses: RentalList,
    ) -> Street {
        Street {
            name,
            buy_price,
            color,
            has_owner: false,
            house_counter: 0,
            is_mortaged: false,
            owner: None,
            rental_priceses,
            price_per_house,
        }
    }

    pub fn get_buy_price(&self) -> u32 {
        self.buy_price
    }
    pub fn add_house(&mut self) {
        self.house_counter += 1;
    }
    pub fn change_owner(&mut self, new_owner: Player) {
        self.owner = Some(new_owner);
    }
    pub fn get_rental_for(&self, houses: RentalPriceFor) -> u32 {
        self.rental_priceses.get_rental_for(houses)
    }
    //TODO
}
impl FieldType for Street {
    fn get_name(&self) -> &str {
        &self.name
    }
}
