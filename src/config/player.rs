
use std::fmt;

pub mod game;

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
pub struct Strategy {
    name: String,
}

impl Strategy {
    pub fn select_card(&self, prize_card: u32, hand: &Hand, max_card: u32) -> u32 {
        hand.cards[0]
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub strategy: Strategy,
    pub player_stats: PlayerStats,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: String::from("unknown"),
            hand: Hand::new(),
            strategy: Strategy{name: String::from("next_card")},
            player_stats: PlayerStats::new(),
        }
    }

    pub fn get_bid(&mut self, prize_card: u32, max_card: u32) -> Bid {
        let offer = self.strategy.select_card(prize_card, &self.hand, max_card);
        self.hand.cards.retain(|x| *x != offer);
        Bid{prize_card: prize_card, offer: offer, bidder: self}
    }

    pub fn wins_round(&mut self, prize_card: u32) { self.player_stats.wins_round(prize_card); }
    pub fn loses_round(&mut self) { self.player_stats.loses_round(); }
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

    fn loses_round(&mut self) {
        self.total_for_game = 0;
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
        write!(f, "({}, {}, {})", self.num_games_won, self.num_rounds_won, self.total_for_game)
    }
}

mod tests {
    use super::*;

	#[test]
	fn test_canary() {
        let a = 5;
        let b = 5;
		assert_eq!(a, b);
	}
}
