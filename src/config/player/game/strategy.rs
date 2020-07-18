
use super::super::Hand;

#[derive(Debug)]
pub struct Strategy {
    pub name: String,
}

impl Strategy {
    pub fn select_card(&self, _prize_card: u32, hand: &Hand, _max_card: u32) -> u32 {
        hand.cards[0]
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
