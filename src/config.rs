
use serde_json;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs;

use player::game::Table;
use player::*;

pub mod player;

#[derive(Debug)]
pub struct Config {
    pub num_players: u32,
    pub num_games: u32,
    pub num_cards: u32,
    pub num_cards_per_hand: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonConfiguration {
    pub num_cards: u32,
    pub num_games: u32,
    pub players: Vec<JsonPlayer>
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonPlayer {
    pub name: String,
    pub strategy: String,
}

pub fn build_from_json(config_file: &str) -> (Table, Config) {
    let data = fs::read_to_string(config_file).expect("Unable to read file");
    let json_configuration: JsonConfiguration = serde_json::from_str(&data).unwrap();

    let mut players: Vec<Player> = vec![];

    for json_player in json_configuration.players {
        let player = Player::new_from_json(&json_player.name, &json_player.strategy);
        players.push(player);
    }

    let num_players = u32::try_from(players.len()).unwrap();
    let table = Table{players: players, .. Table::new()};

    let num_games = json_configuration.num_games;
    let num_cards = json_configuration.num_cards;
    let num_cards_per_hand = num_cards / (num_players + 1);
    let config = Config{num_players: num_players, num_games: num_games,
                        num_cards: num_cards, num_cards_per_hand: num_cards_per_hand};

    (table, config)
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
