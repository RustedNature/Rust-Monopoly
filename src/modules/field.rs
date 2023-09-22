use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

use super::{color::Color, player::Player};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldType {
    Chance {
        name: String,
        //cards: Vec<Cards>,
    },
    CommunityChest {
        name: String,
        //cards: Vec<Cards>
    },
    FreeParking {
        name: String,
    },
    Go {
        name: String,
        income: u32,
    },
    GoToJail {
        name: String,
    },
    InJail {
        name: String,
    },
    Railroad {
        name: String,
        buy_price: u32,
        owner: Option<Player>,
        base_rent: u32,
    },
    Street {
        name: String,
        buy_price: u32,
        num_of_houses: u8,
        price_one_house: u32,
        rental_prices: [u32; 6],
        color: Color,
        owner: Option<Player>,
    },
    Tax {
        name: String,
        tax: u32,
    },
    Utility {
        buy_price: u32,
        name: String,
        owner: Option<Player>,
    },
}

const MAX_STREET_CNT_GREEN_AND_DARKBLUE: u32 = 2;
const MAX_STREET_CNT_OTHER: u32 = 3;
const MAX_UTILITY_CNT: u32 = 2;
const UTILITY_MULTIPLIER_ONE: u32 = 4;
const UTILITY_MULTIPLIER_TWO: u32 = 10;
impl FieldType {
    pub fn get_name(&self) -> String {
        match self {
            FieldType::Go { name, .. } => name.clone(),
            FieldType::Tax { name, .. } => name.clone(),
            FieldType::Street { name, .. } => name.clone(),
            FieldType::Railroad { name, .. } => name.clone(),
            FieldType::CommunityChest { name } => name.clone(),
            FieldType::Chance { name } => name.clone(),
            FieldType::InJail { name } => name.clone(),
            FieldType::Utility { name, .. } => name.clone(),
            FieldType::FreeParking { name } => name.clone(),
            FieldType::GoToJail { name } => name.clone(),
        }
    }

    fn get_num_of_houses(&self) -> Option<&u8> {
        match self {
            FieldType::Street { num_of_houses, .. } => Some(num_of_houses),
            _ => None,
        }
    }
    pub fn get_buy_price(&self) -> Option<u32> {
        match self {
            FieldType::Street { buy_price, .. }
            | FieldType::Railroad { buy_price, .. }
            | FieldType::Utility { buy_price, .. } => Some(buy_price.clone()),
            _ => None,
        }
    }
    pub fn get_rental_price(&self, current_player: &Player) -> Option<u32> {
        match self {
            FieldType::Street {
                rental_prices,
                num_of_houses,
                color,
                owner,
                ..
            } => {
                calc_rent_for_streets(&owner.clone().unwrap(), color, num_of_houses, rental_prices)
            }

            FieldType::Railroad {
                owner, base_rent, ..
            } => {
                let mut rental = base_rent.clone();

                for i in 2..=owner.clone().unwrap().get_num_of_owned_rail_roades() {
                    rental = rental * 2;
                }
                Some(rental)
            }
            FieldType::Utility { owner, .. } => {
                let owned_utilitys = owner.clone().unwrap().get_num_of_owned_utilitiys();
                let current_player_dice_roll = current_player.get_current_dice_roll();
                if owned_utilitys == MAX_UTILITY_CNT {
                    Some(UTILITY_MULTIPLIER_TWO * current_player_dice_roll)
                } else {
                    Some(UTILITY_MULTIPLIER_ONE * current_player_dice_roll)
                }
            }
            _ => None,
        }
    }

    pub(crate) fn is_buyable(&self) -> bool {
        match self {
            FieldType::Street { .. } => true,
            FieldType::Railroad { .. } => true,
            FieldType::Utility { .. } => true,
            _ => false,
        }
    }
}

fn calc_rent_for_streets(
    owner: &Player,
    color: &Color,
    num_of_houses: &u8,
    rental_prices: &[u32; 6],
) -> Option<u32> {
    let owned_streets_cnt_of_specific_color = owner.get_num_of_owned_streets_of_one_color(color);
    if num_of_houses.clone() == 0 {
        calc_rent_without_houses(color, owned_streets_cnt_of_specific_color, rental_prices)
    } else {
        Some(rental_prices[num_of_houses.clone() as usize])
    }
}

fn calc_rent_without_houses(
    color: &Color,
    owned_streets_cnt_of_specific_color: u32,
    rental_prices: &[u32; 6],
) -> Option<u32> {
    match color {
        Color::Brown | Color::DarkBlue => {
            if owned_streets_cnt_of_specific_color == MAX_STREET_CNT_GREEN_AND_DARKBLUE {
                Some(rental_prices[0] * 2)
            } else {
                Some(rental_prices[0])
            }
        }
        _ => {
            if owned_streets_cnt_of_specific_color == MAX_STREET_CNT_OTHER {
                Some(rental_prices[0] * 2)
            } else {
                Some(rental_prices[0])
            }
        }
    }
}
