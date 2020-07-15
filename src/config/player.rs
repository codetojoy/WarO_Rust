
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
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: String::from("unknown"),
            hand: Hand::new(),
            strategy: Strategy{name: String::from("next_card")},
        }
    }

    pub fn get_bid(&mut self, prize_card: u32, max_card: u32) -> Bid {
        let offer = self.strategy.select_card(prize_card, &self.hand, max_card);
        self.hand.cards.retain(|x| *x != offer);
        Bid{prize_card: prize_card, offer: offer, bidder: self}
    }
}

#[derive(Debug)]
pub struct Bid<'a> {
    pub offer: u32,
    pub bidder: &'a Player,
    pub prize_card: u32,
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
