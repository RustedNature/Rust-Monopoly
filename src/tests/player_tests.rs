#[cfg(test)]
mod tests {
    use crate::modules::player::{self, Player};
    #[test]
    fn test_10_000_roll_the_dice() {
        let mut player = Player::new("Test".to_string(), false);

        for i in 0..=10_000 {
            player.roll_the_dice();
            let dice_roll = player.get_current_dice_roll();
            assert!(
                dice_roll >= 2 && dice_roll <= 12,
                "invalid roll {}",
                dice_roll
            );
        }
    }
}
