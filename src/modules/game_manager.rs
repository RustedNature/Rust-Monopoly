use std::{fmt::format, io, thread, time::Duration};

use super::{
    bank::Bank,
    board::Board,
    constants::constant::{MAX_PLAYERS, MIN_PLAYERS},
    io_manager,
    player::{self, Player},
};

pub struct GameManager {
    players: Vec<Player>,
    bank: Bank,
    board: Board,
    game_over: bool,
    round_cnt: u32,
    player_cnt: u32,
    cpu_player_cnt: u32,
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}
impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            players: Vec::new(),
            bank: Bank::new(),
            board: Board::new(),
            game_over: false,
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
        //TODO figurine
        self.start_game();
    }

    fn start_game(&mut self) {
        let mut players = self.players.clone();
        loop {
            self.round_cnt += 1;
            for p in &mut players {
                io_manager::write_header(p.get_name());
                io_manager::notify_player_to_take_action(p.get_name());
                io_manager::display_player_actions();
                io_manager::write_console("Bitte wähle eine Aktion aus: ");
                let mut next_action = io_manager::read_console();
                match next_action.as_str() {
                    "1" => {
                        io_manager::write_line_console(format!(
                            "Spieler {} hat die Aktion würfeln ausgewählt",
                            p.get_name()
                        ));
                        p.roll_the_dice();
                        p.move_fields_based_on_dice_roll();
                    }
                    "2" => {} //TODO Handeln
                    "3" => {} //TODO Haus kaufen
                    "4" => {} //TODO Haus verkaufen
                    "5" => {} //TODO Feld beleihen
                    _ => {}   //TODO Wenn nichts passt
                }

                if self.game_over {
                    break;
                }
            }
        }
    }

    fn get_num_player_from_user_input(&mut self) {
        loop {
            for i in MIN_PLAYERS..=MAX_PLAYERS {
                io_manager::write_line_console(format!("[{}] Spieler", i));
            }
            io_manager::write_console("Bitte geben Sie die Anzahl der Spieler an: ");
            let input = io_manager::read_console();
            let parsed_player_cnt: u32 = input.parse().unwrap_or(0);

            if (MIN_PLAYERS..=MAX_PLAYERS).contains(&parsed_player_cnt) {
                self.player_cnt = parsed_player_cnt;
                break;
            }
        }
    }

    fn get_num_cpu_player_from_user_input(&mut self) {
        loop {
            for i in 0..=self.player_cnt {
                io_manager::write_line_console(format!("[{}] Cpu-Spieler", i).as_str());
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
        io_manager::write_line_console(
            "Alle Spieler würfeln nun um die Startreihenfolge zu ermitteln",
        );
        thread::sleep(Duration::from_secs(3));
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

            (0..roll_per_player.len()).for_each(|index| {
                if roll_per_player[index] > highroll {
                    highroll = roll_per_player[index];
                    highroll_index = index;
                }
            });

            reorderd_players.push(self.players[highroll_index].clone());
            self.players.remove(highroll_index);
            roll_per_player.remove(highroll_index);
        }

        self.players = reorderd_players;
    }

    fn list_reordered_players(&mut self) {
        let mut starts_at = 1;
        for p in &mut self.players {
            io_manager::notify_players_play_order(
                p.get_name(),
                p.get_current_dice_roll(),
                starts_at,
            );
            starts_at += 1;
        }
        thread::sleep(Duration::from_secs(5));
    }

    fn evaluate_next_action(&mut self, next_action: &str, player: &mut Player) {}

    fn action_roll_dice_and_move(&self, player: &mut Player) {}
}
