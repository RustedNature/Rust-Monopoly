use std::{
    any::{Any, TypeId},
    fs::DirEntry,
};

use crate::modules::{bank::Bank, color::Color, player::Player};

use super::{constants::constant::CNT_OF_FIELDS_ON_BOARD, field::FieldType};

#[derive(PartialEq, Eq)]
pub struct Board {
    fields: [FieldType; 40],
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: create_fields(),
        }
    }
    pub fn get_field_name(&self, field_index: u32) -> &str {
        self.fields[field_index as usize].get_name()
    }

    pub(crate) fn get_field(&self, field_index: u32) -> FieldType {
        self.fields[field_index as usize].clone()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

fn create_fields() -> [FieldType; 40] {
    [
        FieldType::Go {
            name: "Los".to_string(),
            income: 200,
        },
        FieldType::Street {
            name: "Badstraße".to_string(),
            buy_price: 60,
            num_of_houses: 0,
            rental_prices: [2, 10, 30, 90, 160, 250],
            color: Color::Brown,
            owner: None,
            price_one_house: 50,
        },
        FieldType::CommunityChest {
            name: "Gemeinschaftsfeld".to_string(),
        },
        FieldType::Street {
            name: "Turmstraße".to_string(),
            buy_price: 60,
            num_of_houses: 0,
            rental_prices: [4, 20, 60, 180, 320, 450],
            color: Color::Brown,
            owner: None,
            price_one_house: 50,
        },
        FieldType::Tax {
            name: "Einkommenssteuer".to_string(),
            tax: 200,
        },
        FieldType::Railroad {
            name: "Südbahnhof".to_string(),
            buy_price: 200,
            owner: None,
            base_rent: 50,
        },
        FieldType::Street {
            name: "Chausseestraße".to_string(),
            buy_price: 100,
            num_of_houses: 0,
            price_one_house: 50,
            rental_prices: [6, 30, 90, 270, 400, 550],
            color: Color::LightBlue,
            owner: None,
        },
        FieldType::Chance {
            name: "Ereignisfeld".to_string(),
        },
        FieldType::Street {
            name: "Elisenstraße".to_string(),
            buy_price: 100,
            num_of_houses: 0,
            price_one_house: 50,
            rental_prices: [6, 30, 90, 270, 400, 550],
            color: Color::LightBlue,
            owner: None,
        },
        FieldType::Street {
            name: "Poststraße".to_string(),
            buy_price: 120,
            num_of_houses: 0,
            price_one_house: 50,
            rental_prices: [8, 40, 100, 300, 450, 600],
            color: Color::LightBlue,
            owner: None,
        },
        FieldType::InJail {
            name: "Zu Besuch / Im Gefängnis".to_string(),
        },
        FieldType::Street {
            name: "Seestraße".to_string(),
            buy_price: 140,
            num_of_houses: 0,
            price_one_house: 100,
            rental_prices: [10, 50, 150, 450, 625, 750],
            color: Color::Pink,
            owner: None,
        },
        FieldType::Utility {
            buy_price: 150,
            name: "Elektrizitätswerk".to_string(),
            owner: None,
        },
        FieldType::Street {
            name: "Hafenstraße".to_string(),
            buy_price: 140,
            num_of_houses: 0,
            price_one_house: 100,
            rental_prices: [10, 50, 150, 450, 625, 750],
            color: Color::Pink,
            owner: None,
        },
        FieldType::Street {
            name: "Neue Straße".to_string(),
            buy_price: 160,
            num_of_houses: 0,
            price_one_house: 100,
            rental_prices: [12, 60, 180, 500, 700, 900],
            color: Color::Pink,
            owner: None,
        },
        FieldType::Railroad {
            name: "Westbahnhof".to_string(),
            buy_price: 200,
            owner: None,
            base_rent: 50,
        },
        FieldType::Street {
            name: "Müncher Straße".to_string(),
            buy_price: 180,
            num_of_houses: 0,
            price_one_house: 100,
            rental_prices: [14, 70, 200, 550, 750, 950],
            color: Color::Orange,
            owner: None,
        },
        FieldType::CommunityChest {
            name: "Gemeinschaftsfeld".to_string(),
        },
        FieldType::Street {
            name: "Wiener Straße".to_string(),
            buy_price: 180,
            num_of_houses: 0,
            price_one_house: 100,
            rental_prices: [14, 70, 200, 550, 750, 950],
            color: Color::Orange,
            owner: None,
        },
        FieldType::Street {
            name: "Berliner Straße".to_string(),
            buy_price: 200,
            num_of_houses: 0,
            price_one_house: 100,
            rental_prices: [16, 80, 220, 600, 800, 1000],
            color: Color::Orange,
            owner: None,
        },
        FieldType::FreeParking {
            name: "Frei Parken".to_string(),
        },
        FieldType::Street {
            name: "Theaterstraße".to_string(),
            buy_price: 220,
            num_of_houses: 0,
            price_one_house: 150,
            rental_prices: [18, 90, 250, 700, 875, 1050],
            color: Color::Red,
            owner: None,
        },
        FieldType::Chance {
            name: "Ereignisfeld".to_string(),
        },
        FieldType::Street {
            name: "Museumstraße".to_string(),
            buy_price: 220,
            num_of_houses: 0,
            price_one_house: 150,
            rental_prices: [18, 90, 250, 700, 875, 1050],
            color: Color::Red,
            owner: None,
        },
        FieldType::Street {
            name: "Opernplatz".to_string(),
            buy_price: 240,
            num_of_houses: 0,
            price_one_house: 150,
            rental_prices: [20, 110, 330, 750, 925, 1100],
            color: Color::Red,
            owner: None,
        },
        FieldType::Railroad {
            name: "Nordbahnhof".to_string(),
            buy_price: 200,
            owner: None,
            base_rent: 50,
        },
        FieldType::Street {
            name: "Lessingstraße".to_string(),
            buy_price: 260,
            num_of_houses: 0,
            price_one_house: 150,
            rental_prices: [22, 110, 330, 800, 975, 1150],
            color: Color::Yellow,
            owner: None,
        },
        FieldType::Street {
            name: "Schillerstraße".to_string(),
            buy_price: 260,
            num_of_houses: 0,
            price_one_house: 150,
            rental_prices: [22, 110, 330, 800, 975, 1150],
            color: Color::Yellow,
            owner: None,
        },
        FieldType::Utility {
            buy_price: 150,
            name: "Wasserwerk".to_string(),
            owner: None,
        },
        FieldType::Street {
            name: "Goethestraße".to_string(),
            buy_price: 280,
            num_of_houses: 0,
            price_one_house: 150,
            rental_prices: [24, 120, 360, 850, 1025, 1200],
            color: Color::Yellow,
            owner: None,
        },
        FieldType::GoToJail {
            name: "Gehe in das gefängnis".to_string(),
        },
        FieldType::Street {
            name: "Rathausplatz".to_string(),
            buy_price: 300,
            num_of_houses: 0,
            price_one_house: 200,
            rental_prices: [26, 130, 390, 900, 1100, 1275],
            color: Color::Green,
            owner: None,
        },
        FieldType::Street {
            name: "Hauptstraße".to_string(),
            buy_price: 300,
            num_of_houses: 0,
            price_one_house: 200,
            rental_prices: [24, 120, 360, 850, 1025, 1200],
            color: Color::Green,
            owner: None,
        },
        FieldType::CommunityChest {
            name: "Gemeinschaftsfeld".to_string(),
        },
        FieldType::Street {
            name: "Bahnhofstraße".to_string(),
            buy_price: 320,
            num_of_houses: 0,
            price_one_house: 200,
            rental_prices: [28, 150, 450, 1000, 1200, 1400],
            color: Color::Green,
            owner: None,
        },
        FieldType::Railroad {
            name: "Hauptbahnhof".to_string(),
            buy_price: 200,
            owner: None,
            base_rent: 50,
        },
        FieldType::Chance {
            name: "Ereignisfeld".to_string(),
        },
        FieldType::Street {
            name: "Parkstraße".to_string(),
            buy_price: 350,
            num_of_houses: 0,
            price_one_house: 200,
            rental_prices: [35, 175, 500, 1100, 1300, 1500],
            color: Color::DarkBlue,
            owner: None,
        },
        FieldType::Tax {
            name: "Luxussteuer".to_string(),
            tax: 100,
        },
        FieldType::Street {
            name: "Schlossallee".to_string(),
            buy_price: 400,
            num_of_houses: 0,
            price_one_house: 200,
            rental_prices: [50, 200, 600, 1400, 1700, 2000],
            color: Color::DarkBlue,
            owner: None,
        },
    ]
}
