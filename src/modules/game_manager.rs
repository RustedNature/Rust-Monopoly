use std::{cell::RefCell, fmt::format, io, rc::Rc, thread, time::Duration};

use super::{
    bank::Bank,
    board::Board,
    constants::constant::{MAX_PLAYERS, MIN_PLAYERS},
    field::FieldType,
    io_manager,
    player::{self, Player, PlayerType},
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
        loop {
            self.round_cnt += 1;
            for mut player in self.players.iter_mut() {
                {
                    self.process_player_turn(player);
                    if self.game_over {
                        break;
                    }
                }
            }
        }
    }

    fn process_player_turn(&mut self, player: &mut Player) {
        io_manager::write_header(player.get_name());
        io_manager::notify_player_to_take_action(player.get_name());
        io_manager::display_player_actions();
        io_manager::write_console("Bitte wähle eine Aktion aus: ");
        let mut next_action = io_manager::read_console();
        match next_action.as_str() {
            "1" => {
                self.action_roll_dice_and_move(player);
            }
            "2" => {} //TODO Handeln
            "3" => {} //TODO Haus kaufen
            "4" => {} //TODO Haus verkaufen
            "5" => {} //TODO Feld beleihen
            _ => {}   //TODO Wenn nichts passt
        }
    }

    fn action_roll_dice_and_move(&mut self, player: &mut Player) {
        io_manager::write_line_console(format!(
            "Spieler {} hat die Aktion würfeln ausgewählt",
            player.get_name()
        ));
        player.roll_the_dice();
        player.move_fields_based_on_dice_roll();
        io_manager::write_line_console(format!(
            "Spieler {} hat eine {} gewürfelt und ist von {} nach {} gegangen",
            player.get_name(),
            player.get_current_dice_roll(),
            self.board.get_field_name(player.get_last_position()),
            self.board.get_field_name(player.get_current_position())
        ));
        let field = self.board.get_field(player.get_current_position());
        match field {
            FieldType::Street { owner, .. }
            | FieldType::Railroad { owner, .. }
            | FieldType::Utility { owner, .. } => match self.is_valid_owner(owner) {
                true => {
                    io_manager::write_line_console(format!(
                        "Das Grundstück {} gehört dem Spieler {}!",
                        field.get_name(),
                        owner.unwrap().get_name()
                    ));
                    self.process_pay_rent(player, owner, field);
                }
                false => {
                    io_manager::write_line_console(format!(
                        "Das Grundstück {} gehört noch niemanden!",
                        field.get_name()
                    ));
                    self.process_buy_property(player, field);
                }
            },

            FieldType::Tax { tax, .. } => player.pay_tax(tax),
            FieldType::GoToJail { .. } => player.go_in_jail(),
            FieldType::CommunityChest { .. } => (), //TODO Draw card,
            FieldType::Chance { .. } => (),         //TODO Draw card,
            _ => (),
        }
    }

    fn process_pay_rent(
        &mut self,
        player: &mut Player,
        owner: Option<Player>,
        property: FieldType,
    ) {
        io_manager::write_line_console(format!(
            "Spieler {} muss dem Spieler {} {}€ zahlen!",
            player.get_name(),
            owner.clone().unwrap().get_name(),
            property.get_rental_price(player).unwrap(),
        ));
        player.pay_money_to_player(
            owner.unwrap(),
            property.get_rental_price(player).unwrap() as i32,
        )
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
            let mut cpu_player = Player::new(cpu_player_name, PlayerType::CpuPlayer);
            self.players.push(cpu_player);
        }
    }

    fn fill_human_players(&mut self) {
        for i in 1..=self.player_cnt - self.cpu_player_cnt {
            io_manager::write_console(
                format!("Bitte geben Sie einen Namen für den Spieler {}: ", i).as_str(),
            );
            let player_name = io_manager::read_console();
            let mut player = Player::new(player_name, PlayerType::HumanPlayer);
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
        let mut reordered_players = self.players.clone();
        reordered_players.sort_by_key(|player| std::cmp::Reverse(player.get_current_dice_roll()));
        self.players = reordered_players;
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

    fn is_valid_owner(&self, owner: Option<Player>) -> bool {
        match owner {
            Some(owner) => true,
            None => false,
        }
    }

    fn process_buy_property(&self, player: &mut Player, property: FieldType) {
        let mut want_to_buy = false;
        io_manager::write_line_console(format!(
            "Möchtest du {} das Grundstück {} für {}€ kaufen?",
            player.get_name(),
            property.get_name(),
            property.get_buy_price().unwrap(),
        ));
        let input = io_manager::read_console();
    }
}
