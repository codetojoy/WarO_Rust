
use super::super::Hand;

// this should be a trait !!??
#[derive(Debug)]
pub struct Strategy {
    pub name: String,
}

const CONSOLE_CARD: &str = "console_card";
const MAX_CARD: &str = "max_card";
const MIN_CARD: &str = "min_card";
const NEXT_CARD: &str = "next_card";

impl Strategy {
    pub fn select_card(&self, prize_card: u32, hand: &Hand, highest_card: u32) -> u32 {

        match self.name.as_str() {
            CONSOLE_CARD => console_card(prize_card, hand, highest_card),
            MAX_CARD => max_card(prize_card, hand, highest_card),
            MIN_CARD => min_card(prize_card, hand, highest_card),
            NEXT_CARD => next_card(prize_card, hand, highest_card),
            _ => next_card(prize_card, hand, highest_card),
        }
    }
}

fn console_card(_prize_card: u32, hand: &Hand, _max_card: u32) -> u32 {
    hand.cards[0]
}

fn max_card(_prize_card: u32, hand: &Hand, _max_card: u32) -> u32 {
    *hand.cards.iter().max().unwrap()
}

fn min_card(_prize_card: u32, hand: &Hand, _max_card: u32) -> u32 {
    *hand.cards.iter().min().unwrap()
}

fn next_card(_prize_card: u32, hand: &Hand, _max_card: u32) -> u32 {
    hand.cards[0]
}

#[allow(unused_imports)]
mod tests {
    use super::*;

	#[test]
	fn test_max_card() {
        let prize_card = 10;
        let hand = Hand{cards: vec![4,2,6,8]};
        let highest_card = 12;
        // test
        let result = max_card(prize_card, &hand, highest_card);

		assert_eq!(result, 8);
	}

	#[test]
	fn test_min_card() {
        let prize_card = 10;
        let hand = Hand{cards: vec![4,2,6,8]};
        let highest_card = 12;
        // test
        let result = min_card(prize_card, &hand, highest_card);

		assert_eq!(result, 2);
	}

	#[test]
	fn test_next_card() {
        let prize_card = 10;
        let hand = Hand{cards: vec![4,2,6,8]};
        let highest_card = 12;
        // test
        let result = next_card(prize_card, &hand, highest_card);

		assert_eq!(result, 4);
	}
}
