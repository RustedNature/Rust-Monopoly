use crate::modules::{field::Buyable, player::Player};

#[derive(Clone, PartialEq, Eq)]
pub struct RailRoad {
    name: String,
    buy_cost: i32,
    has_owner: bool,
    owner: Option<Player>,
    color: Buyable,
    rent_of_one: i32,
}

impl RailRoad {
    pub fn new(name: String) -> RailRoad {
        RailRoad {
            name,
            buy_cost: 200,
            has_owner: false,
            owner: None,
            color: Buyable::TrainStation,
            rent_of_one: 25,
        }
    }
    //TODO
}
