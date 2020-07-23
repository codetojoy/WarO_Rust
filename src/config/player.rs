
use std::fmt;

pub mod game;

use game::strategy::Strategy;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<u32>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: vec![],
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        s.push_str("[");
        let mut counter = 1;
        let max = self.cards.len();
        for card in &self.cards {
            s.push_str(&card.to_string());
            if counter < max {
                s.push_str(", ");
            }

            counter += 1;
        }
        s.push_str("]");

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Hand,
    strategy: Strategy,
    player_stats: PlayerStats,
}

impl Player {

    // this is used by tests
    #[allow(dead_code)]
    pub fn new() -> Player {
        Player {
            name: String::from("unknown"),
            hand: Hand::new(),
            strategy: Strategy{name: String::from("next_card")},
            player_stats: PlayerStats::new(),
        }
    }

    pub fn new_from_json(name: &str, strategy_name: &str) -> Player {
        Player {
            name: String::from(name),
            hand: Hand::new(),
            strategy: Strategy{name: String::from(strategy_name)},
            player_stats: PlayerStats::new(),
        }
    }

    pub fn get_bid(&mut self, prize_card: u32, max_card: u32) -> Bid {
        let offer = self.strategy.select_card(prize_card, &self.hand, max_card);
        self.hand.cards.retain(|x| *x != offer);
        Bid{prize_card: prize_card, offer: offer, bidder: self}
    }

    pub fn wins_round(&mut self, prize_card: u32) { self.player_stats.wins_round(prize_card); }
    pub fn wins_game(&mut self) { self.player_stats.wins_game(); }
    pub fn loses_game(&mut self) { self.player_stats.loses_game(); }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.hand, self.player_stats)
    }
}

#[derive(Debug)]
pub struct Bid<'a> {
    pub offer: u32,
    pub bidder: &'a Player,
    pub prize_card: u32,
}

impl fmt::Display for Bid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let player = self.bidder;
        let name = &player.name;
        let offer = self.offer;
        write!(f, "player: {:?} bids {:?} on {:?} with hand: {}", name, offer, self.prize_card, player.hand)
    }
}

#[derive(Debug)]
pub struct PlayerStats {
    pub total_for_game: u32,
    pub num_games_won: u32,
    pub num_rounds_won: u32,
}

impl PlayerStats {
    fn new() -> PlayerStats {
        PlayerStats {
            total_for_game: 0,
            num_games_won: 0,
            num_rounds_won: 0,
        }
    }

    fn wins_round(&mut self, prize_card: u32) {
        self.total_for_game += prize_card;
        self.num_rounds_won += 1;
    }

    fn wins_game(&mut self) {
        self.total_for_game = 0;
        self.num_rounds_won = 0;
        self.num_games_won += 1;
    }

    fn loses_game(&mut self) {
        self.total_for_game = 0;
        self.num_rounds_won = 0;
    }
}

impl fmt::Display for PlayerStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(g: {}, r: {}, t: {})", self.num_games_won, self.num_rounds_won, self.total_for_game)
    }
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
