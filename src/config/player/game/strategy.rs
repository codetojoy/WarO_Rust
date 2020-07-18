
use super::super::Hand;

// this should be a trait !!??
#[derive(Debug)]
pub struct Strategy {
    pub name: String,
}

const NEXT_CARD: &str = "next_card";

impl Strategy {
    pub fn select_card(&self, prize_card: u32, hand: &Hand, max_card: u32) -> u32 {

        match self.name.as_str() {
            NEXT_CARD => next_card(prize_card, hand, max_card),
            _ => next_card(prize_card, hand, max_card),
        }
    }
}

fn next_card(_prize_card: u32, hand: &Hand, _max_card: u32) -> u32 {
    hand.cards[0]
}

#[allow(unused_imports)]
mod tests {
    use super::*;

	#[test]
	fn test_next_card() {
        let prize_card = 10;
        let hand = Hand{cards: vec![2,4,6,8]};
        let max_card = 12;
        // test
        let result = next_card(prize_card, &hand, max_card);

		assert_eq!(result, 2);
	}
}
