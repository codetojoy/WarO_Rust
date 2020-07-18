
pub mod player;

#[derive(Debug)]
pub struct Config {
    pub num_players: u32,
    pub num_games: u32,
    pub num_cards: u32,
    pub num_cards_per_hand: u32,
}

#[allow(unused_imports)]
mod tests {
    use super::*;

	#[test]
	fn test_canary() {
        let a = 5;
        let b = 5;
		assert_eq!(a, b);
	}
}
