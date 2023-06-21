use crate::modules::{color::Color, player::Player};
pub struct Street {
    name: String,
    has_owner: bool,
    buy_price: u32,
    house_counter: u8,
    is_mortaged: bool,
    owner: Option<Player>,
    rental_priceses: Vec<u32>,
    price_per_house: u32,
    color: Color,
}

impl Street {
    pub fn new(
        name: String,
        color: Color,
        buy_price: u32,
        rental_price_zero_houses: u32,
        rental_price_one_house: u32,
        rental_price_two_houses: u32,
        rental_price_three_houses: u32,
        rental_price_four_houses: u32,
        rental_price_hotel: u32,
        price_per_house: u32,
    ) -> Street {
        let rental_priceses: Vec<u32> = vec![
            rental_price_zero_houses,
            rental_price_one_house,
            rental_price_two_houses,
            rental_price_three_houses,
            rental_price_four_houses,
            rental_price_hotel,
        ];

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
    //TODO
}
