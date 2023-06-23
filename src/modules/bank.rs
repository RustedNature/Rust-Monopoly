use crate::modules::{constants::constants::*, player::Player};
#[derive(Clone, PartialEq, Eq)]
pub struct Bank {
    money: i32,
    free_parking_money: i32,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            money: BANK_START_MONEY,
            free_parking_money: 0,
        }
    }

    pub fn pay_in(&mut self, amount: i32) {
        self.money += amount;
    }
    pub fn pay_out(&mut self, amount: i32, player: &mut Player) {
        self.money -= amount;
        player.receive_money(amount);
    }
    pub fn pay_in_free_parking(&mut self, amount: i32) {
        self.free_parking_money += amount;
    }
    pub fn pay_out_free_parking(&mut self, player: &mut Player) {
        player.receive_money(self.free_parking_money);
        self.free_parking_money = 0;
    }
}
