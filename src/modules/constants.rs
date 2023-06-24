pub mod constant {
    pub const PLAYER_START_MONEY: i32 = 1500;
    pub const GO_FIELD: u32 = 0;
    pub const JAIL_FIELD: u32 = 10;
    pub const BANK_START_MONEY: i32 = 100_000;
    pub const CNT_OF_FIELDS_ON_BOARD: u32 = 39;
    pub const MIN_PLAYERS: u32 = 2;
    pub const MAX_PLAYERS: u32 = 8;
    pub const PLAYER_ACTIONS: [&str; 5] = [
        "Würfeln",
        "Handeln",
        "Häuser kaufen",
        "Häuser verkaufen",
        "Felder beleihen",
    ];
}
