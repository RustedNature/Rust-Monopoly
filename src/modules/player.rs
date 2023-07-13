use rand::Rng;

use crate::modules::constants::constant::*;
use crate::modules::field::*;
use core::num;
use std::collections::HashMap;

use super::color::Color;
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PlayerType {
    CpuPlayer,
    HumanPlayer,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Player {
    name: String,
    money: i32,
    is_in_jail: bool,
    current_dice_roll: u32,
    rounds_in_jail: u32,
    current_position: u32,
    last_position: u32,
    player_type: PlayerType,
    owned_streets: Vec<FieldType>,
}

impl Player {
    pub fn get_current_dice_roll(&self) -> u32 {
        self.current_dice_roll
    }

    pub fn get_current_position(&self) -> u32 {
        self.current_position
    }

    pub fn get_last_position(&self) -> u32 {
        self.last_position
    }

    pub fn get_money(&self) -> i32 {
        self.money
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_num_of_owned_field_types(&self, field_types: &FieldType) -> u32 {
        match field_types {
            FieldType::Street { color, .. } => 0,
            FieldType::Railroad { .. } => self.get_num_of_owned_rail_roades(),
            _ => 0,
        }
    }
    pub fn get_num_of_owned_rail_roades(&self) -> u32 {
        let mut cnt = 0;
        for field_type in &self.owned_streets {
            if let FieldType::Railroad { .. } = field_type {
                cnt += 1;
            }
        }
        cnt
    }
    pub fn get_num_of_owned_streets_of_one_color(&self, color: &Color) -> u32 {
        let mut cnt = 0;
        for field_type in &self.owned_streets {
            if let FieldType::Street {
                color: color_owned, ..
            } = field_type
            {
                if color_owned == color {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    pub fn get_num_of_owned_utilitiys(&self) -> u32 {
        let mut cnt = 0;
        for field_type in &self.owned_streets {
            match field_type {
                FieldType::Utility { .. } => {
                    cnt += 1;
                }
                _ => (),
            }
        }
        cnt
    }
    pub fn go_in_jail(&mut self) {
        self.current_position = JAIL_FIELD;
        self.is_in_jail = true;
    }

    pub fn move_fields_based_on_dice_roll(&mut self) {
        self.last_position = self.current_position;
        self.current_position = (self.current_position + self.current_dice_roll) % 39;
    }

    pub fn new(name: String, player_type: PlayerType) -> Player {
        Player {
            name,
            money: PLAYER_START_MONEY,
            is_in_jail: false,
            current_dice_roll: 0,
            rounds_in_jail: 0,
            current_position: GO_FIELD,
            player_type,
            last_position: GO_FIELD,
            owned_streets: Vec::new(),
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

    pub fn release_from_jail(&mut self) {
        self.is_in_jail = false;
    }

    pub fn roll_the_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let dice1: u32 = rng.gen_range(1..=6);
        let dice2: u32 = rng.gen_range(1..=6);

        self.current_dice_roll = dice1 + dice2;
    }

    pub(crate) fn pay_tax(&mut self, tax: u32) {
        self.money -= tax as i32;
    }
}
