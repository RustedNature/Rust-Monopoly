#[derive(Clone, PartialEq, Eq)]
pub struct RentalList {
    rental_list: [u32; 6],
}
#[derive(Clone, PartialEq, Eq)]
pub enum RentalPriceFor {
    ZeroHouses = 0,
    OneHouse = 1,
    TwoHouses = 2,
    ThreeHouses = 3,
    FourHouses = 4,
    Hotel = 5,
}
impl RentalList {
    pub fn new(
        rental_price_zero_houses: u32,
        rental_price_one_house: u32,
        rental_price_two_houses: u32,
        rental_price_three_houses: u32,
        rental_price_four_houses: u32,
        rental_price_hotel: u32,
    ) -> RentalList {
        let rental_list = [
            rental_price_zero_houses,
            rental_price_one_house,
            rental_price_two_houses,
            rental_price_three_houses,
            rental_price_four_houses,
            rental_price_hotel,
        ];
        RentalList { rental_list }
    }

    pub fn get_rental_for(&self, amount_houses: RentalPriceFor) -> u32 {
        match amount_houses {
            RentalPriceFor::ZeroHouses => self.rental_list[0],
            RentalPriceFor::OneHouse => self.rental_list[1],
            RentalPriceFor::TwoHouses => self.rental_list[2],
            RentalPriceFor::ThreeHouses => self.rental_list[3],
            RentalPriceFor::FourHouses => self.rental_list[4],
            RentalPriceFor::Hotel => self.rental_list[5],
        }
    }
}
