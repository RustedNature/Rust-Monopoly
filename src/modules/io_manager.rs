use std::{
    io::{self, Write},
    process::Command,
};

use super::{
    board::{self, Board},
    constants::constant::PLAYER_ACTIONS,
    player::Player,
};

pub fn write_console<T: ToString>(message: T) {
    print!("{}", message.to_string());
    io::stdout().flush();
}
pub fn write_line_console<T: ToString>(message: T) {
    println!("{}", message.to_string());
}

pub fn read_console() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something wasnt right");
    remove_escape_chars(input)
}
fn remove_escape_chars(input: String) -> String {
    let escape_chars = ['\\', '\"', '\'', '\n', '\r', '\t', '\0'];
    let mut result = input;

    for c in escape_chars.iter() {
        result = result.replace(*c, "");
    }

    result
}
pub fn notify_player_to_take_action<T: ToString>(player_name: T) {
    write_line_console(format!(
        "Spieler {} ist an der Reihe, bitte wähle deine Aktion",
        player_name.to_string()
    ));
}
pub fn display_player_actions() {
    let mut i = 1;
    for s in PLAYER_ACTIONS {
        write_line_console(format!("[{}] {}", i, s).as_str());
        i += 1;
    }
}

pub fn notify_players_play_order<T: ToString>(
    player_name: T,
    current_dice_roll: u32,
    order_num: u32,
) {
    write_line_console(format!(
        "Spieler {} würfelte eine {} und wird als nummer {} starten",
        player_name.to_string(),
        current_dice_roll,
        order_num
    ));
}

pub fn move_player_monolog(
    player_name: &str,
    player_roll: u32,
    player_last_field_name: &str,
    player_current_field_name: &str,
) {
    write_line_console(format!(
        "Spieler {} hat eine {} gewürfelt",
        player_name, player_roll
    ));
    write_console(format!(
        "Spieler {} bewegt sich von dem Feld {} nach ",
        player_name, player_last_field_name
    ));
    write_line_console(format!("{}.", player_current_field_name));
}

pub fn write_header(player_name: &str) {
    write_line_console(format!(
        "Spieler {} ------------------------------------------------------",
        player_name
    ));
}
