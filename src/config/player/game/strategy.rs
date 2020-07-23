
use std::io;

use super::super::Hand;

// this should be a trait !!??
#[derive(Debug)]
pub struct Strategy {
    pub name: String,
}

const CONSOLE: &str = "console";
const HYBRID_CARD: &str = "hybrid_card";
const MAX_CARD: &str = "max_card";
const MIN_CARD: &str = "min_card";
const NEAREST_CARD: &str = "nearest_card";
const NEXT_CARD: &str = "next_card";

impl Strategy {
    pub fn select_card(&self, prize_card: u32, hand: &Hand, highest_card: u32) -> u32 {

        match self.name.as_str() {
            CONSOLE => console(prize_card, hand, highest_card),
            HYBRID_CARD => hybrid_card(prize_card, hand, highest_card),
            MAX_CARD => max_card(prize_card, hand, highest_card),
            MIN_CARD => min_card(prize_card, hand, highest_card),
            NEAREST_CARD => nearest_card(prize_card, hand, highest_card),
            NEXT_CARD => next_card(prize_card, hand, highest_card),
            _ => next_card(prize_card, hand, highest_card),
        }
    }
}

fn console(prize_card: u32, hand: &Hand, _highest_card: u32) -> u32 {
    let mut selection = None;
    let mut pick = String::new();
    println!("\nprize_card: {}", prize_card);
    println!("your hand: {}", hand);

    while ! selection.is_some() {
        println!("enter your pick:");
        selection = match io::stdin().read_line(&mut pick) {
            Ok(_) => validate_pick(&pick, hand),
            Err(_) => None,
        }
    }

    selection.unwrap()
}

fn validate_pick(pick: &str, hand: &Hand) -> Option<u32>  {
    let guess: u32 = pick.trim().parse().expect("Please type a number!");
    match hand.cards.contains(&guess) {
        false => None,
        true => Some(guess),
    }
}

fn hybrid_card(prize_card: u32, hand: &Hand, highest_card: u32) -> u32 {
    if prize_card > (highest_card / 2) {
        max_card(prize_card, hand, highest_card)
    } else {
        min_card(prize_card, hand, highest_card)
    }
}

fn nearest_card(prize_card: u32, hand: &Hand, highest_card: u32) -> u32 {
    let (nearest, _tmp) = hand.cards.iter().fold((0, highest_card), |acc, card| -> (u32, u32) {
        let (_nearest_card_so_far, nearest_distance_so_far) = acc;
        let this_distance = (*card as i32 - prize_card as i32).abs() as u32;
        if this_distance < nearest_distance_so_far  {
            (*card, this_distance)
        } else {
            acc
        }
    });
    nearest
}

fn next_card(_prize_card: u32, hand: &Hand, _highest_card: u32) -> u32 {
    hand.cards[0]
}

fn max_card(_prize_card: u32, hand: &Hand, _highest_card: u32) -> u32 {
    *hand.cards.iter().max().unwrap()
}

fn min_card(_prize_card: u32, hand: &Hand, _highest_card: u32) -> u32 {
    *hand.cards.iter().min().unwrap()
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_validate_pick_ok() {
        let pick = "8";
        let hand = Hand{cards: vec![4,2,6,8]};
        // test
        let result = validate_pick(&pick, &hand);

        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_validate_pick_ok_newline() {
        let pick = "8\n";
        let hand = Hand{cards: vec![4,2,6,8]};
        // test
        let result = validate_pick(&pick, &hand);

        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_validate_pick_not_legal() {
        let pick = "@2";
        let hand = Hand{cards: vec![4,2,6,8]};
        // test
        let result = panic::catch_unwind(|| {
            validate_pick(&pick, &hand);
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pick_not_found() {
        let pick = "7";
        let hand = Hand{cards: vec![4,2,6,8]};
        // test
        let result = validate_pick(&pick, &hand);

        assert_eq!(false, result.is_some());
    }

    #[test]
    fn test_hybrid_card_high() {
        let prize_card = 10;
        let hand = Hand{cards: vec![4,2,6,8]};
        let highest_card = 12;
        // test
        let result = hybrid_card(prize_card, &hand, highest_card);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_hybrid_card_low() {
        let prize_card = 1;
        let hand = Hand{cards: vec![4,2,6,8]};
        let highest_card = 12;
        // test
        let result = hybrid_card(prize_card, &hand, highest_card);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_nearest_card_low() {
        let prize_card = 1;
        let hand = Hand{cards: vec![5,6,9,10]};
        let highest_card = 12;
        // test
        let result = nearest_card(prize_card, &hand, highest_card);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_nearest_card_middle() {
        let prize_card = 7;
        let hand = Hand{cards: vec![1,6,9,10]};
        let highest_card = 12;
        // test
        let result = nearest_card(prize_card, &hand, highest_card);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_nearest_card_high() {
        let prize_card = 12;
        let hand = Hand{cards: vec![2,6,9,10]};
        let highest_card = 12;
        // test
        let result = nearest_card(prize_card, &hand, highest_card);

        assert_eq!(result, 10);
    }

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
