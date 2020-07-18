
// use rand::seq::SliceRandom;
use std::convert::TryFrom;

mod config;

use config::Config;
use config::player::game::Table;
use config::player::*;

fn main() {
    // players should go into a config module but not necessarily Config
    let p1 = Player{name: String::from("mozart"), .. Player::new()};
    let p2 = Player{name: String::from("beethoven"), .. Player::new()};
    let p3 = Player{name: String::from("chopin"), .. Player::new()};
    let players: Vec<Player> = vec![p1, p2, p3];
    let num_players = u32::try_from(players.len()).unwrap();
    let mut table = Table{players: players, .. Table::new()};

    const NUM_GAMES: u32 = 1;
    const NUM_CARDS: u32 = 12;
    let num_cards_per_hand = NUM_CARDS / (num_players + 1);
    let config = Config{num_players: num_players, num_games: NUM_GAMES,
                        num_cards: NUM_CARDS, num_cards_per_hand: num_cards_per_hand};

    for _i in 1..20 {
        println!("");
    }
    println!("----------------------------------");
    println!("TRACER config: {:?}", config);
    println!("TRACER table: {:?}", table);
    game::play_tourney(&config, &mut table);
    println!("Ready.");
}
