use std::io;

use super::{
    bank::Bank,
    board::Board,
    constants::constants::{MAX_PLAYERS, MIN_PLAYERS},
    io_manager,
    player::{self, Player},
};

pub struct GameManager {
    players: Vec<Player>,
    bank: Bank,
    board: Board,
    round_cnt: u32,
    player_cnt: u32,
    cpu_player_cnt: u32,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            players: Vec::new(),
            bank: Bank::new(),
            board: Board::new(),
            round_cnt: 0,
            player_cnt: 0,
            cpu_player_cnt: 0,
        }
    }

    pub fn setup_game(&mut self) {
        self.get_num_player_from_user_input();
        self.get_num_cpu_player_from_user_input();
        self.fill_players();
        self.all_player_initial_dice_roll();
        self.reorder_players_based_on_dice_roll();
        self.list_reordered_players();
        //todo figurine
        self.start_game();
    }

    fn start_game(&mut self) {}

    fn get_num_player_from_user_input(&mut self) {
        loop {
            for i in MIN_PLAYERS..=MAX_PLAYERS {
                io_manager::write_line_console(format!("[{}] player", i).as_str());
            }
            io_manager::write_console("Bitte geben Sie die Anzahl der Spieler an: ");
            let input = io_manager::read_console();
            let parsed_player_cnt: u32 = input.parse().unwrap_or(0);

            if parsed_player_cnt >= MIN_PLAYERS && parsed_player_cnt <= MAX_PLAYERS {
                self.player_cnt = parsed_player_cnt;
                break;
            }
        }
    }

    fn get_num_cpu_player_from_user_input(&mut self) {
        loop {
            for i in 0..=self.player_cnt {
                io_manager::write_line_console(format!("[{}] cpu player", i).as_str());
            }
            io_manager::write_console("Bitte geben Sie die Anzal der CPU Spieler an: ");
            let input = io_manager::read_console();
            let parsed_cpu_player_cnt: i32 = input.parse().unwrap_or(500);
            if parsed_cpu_player_cnt >= 0 && parsed_cpu_player_cnt as u32 <= self.player_cnt {
                self.cpu_player_cnt = parsed_cpu_player_cnt as u32;
                break;
            }
        }
    }

    fn fill_players(&mut self) {
        self.fill_human_players();
        self.fill_cpu_players();
    }

    fn fill_cpu_players(&mut self) {
        for i in 1..=self.cpu_player_cnt {
            io_manager::write_console(
                format!("Bitte geben Sie einen Namen für den Cpu-Spieler {}: ", i).as_str(),
            );
            let cpu_player_name = io_manager::read_console();
            let mut cpu_player = Player::new(cpu_player_name, true);
            self.players.push(cpu_player);
        }
    }

    fn fill_human_players(&mut self) {
        for i in 1..=self.player_cnt - self.cpu_player_cnt {
            io_manager::write_console(
                format!("Bitte geben Sie einen Namen für den Spieler {}: ", i).as_str(),
            );
            let player_name = io_manager::read_console();
            let mut player = Player::new(player_name, false);
            self.players.push(player);
        }
    }

    fn all_player_initial_dice_roll(&mut self) {
        for player in &mut self.players {
            player.roll_the_dice();
        }
    }

    fn reorder_players_based_on_dice_roll(&mut self) {
        let mut reorderd_players: Vec<Player> = Vec::new();
        let mut roll_per_player: Vec<u32> = Vec::new();
        for p in &mut self.players {
            roll_per_player.push(p.get_current_dice_roll());
        }

        for i in 0..roll_per_player.len() {
            let mut highroll = 0;
            let mut highroll_index = 0;

            for index in 0..roll_per_player.len() {
                if roll_per_player[index] > highroll {
                    highroll = roll_per_player[index];
                    highroll_index = index;
                }
            }

            reorderd_players.push(self.players[highroll_index].clone());
            self.players.remove(highroll_index);
            roll_per_player.remove(highroll_index);
        }

        self.players = reorderd_players;
    }
}
