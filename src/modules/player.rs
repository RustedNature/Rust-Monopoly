use rand::Rng;

use crate::modules::field::*;
use crate::modules::{color::Color, constants::constant::*};
use core::num;
use std::collections::HashMap;
#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    name: String,
    money: i32,
    is_in_jail: bool,
    is_cpu_player: bool,
    current_dice_roll: u32,
    rounds_in_jail: u32,
    current_position: u32,
    last_position: u32,
    owned_streets: HashMap<Color, Vec<Field>>,
}

impl Player {
    pub fn new(name: String, is_cpu_player: bool) -> Player {
        Player {
            name,
            money: PLAYER_START_MONEY,
            is_in_jail: false,
            is_cpu_player,
            current_dice_roll: 0,
            rounds_in_jail: 0,
            current_position: GO_FIELD,
            last_position: GO_FIELD,
            owned_streets: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_money(&self) -> i32 {
        self.money
    }

    pub fn get_current_dice_roll(&self) -> u32 {
        self.current_dice_roll
    }

    pub fn get_current_position(&self) -> u32 {
        self.current_position
    }
    pub fn get_last_position(&self) -> u32 {
        self.last_position
    }
    pub fn get_number_of_owned_streets_of_specific_color(&self, color: Color) -> u32 {
        if let Some(owned_street_of_one_color) = self.owned_streets.get(&color) {
            owned_street_of_one_color.len() as u32
        } else {
            0
        }
    }

    pub fn pay_money_to_player(&mut self, mut reciepient: Player, amount_to_pay: i32) {
        if self.money >= amount_to_pay {
            self.money -= amount_to_pay;
        } else {
            todo!()
        }

        reciepient.receive_money(amount_to_pay);
    }

    pub fn receive_money(&mut self, amount_to_receive: i32) {
        self.money += amount_to_receive;
    }

    pub fn go_in_jail(&mut self) {
        self.current_position = JAIL_FIELD;
        self.is_in_jail = true;
    }

    pub fn release_from_jail(&mut self) {
        self.is_in_jail = false;
    }

    pub fn move_fields_based_on_dice_roll(&mut self) {
        self.last_position = self.current_position;
        self.current_position = (self.current_position + self.current_dice_roll) % 39;
    }

    pub fn roll_the_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let dice1: u32 = rng.gen_range(1..=6);
        let dice2: u32 = rng.gen_range(1..=6);

        self.current_dice_roll = dice1 + dice2;
    }
}
