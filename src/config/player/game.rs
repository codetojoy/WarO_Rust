
use rand::seq::SliceRandom;
use std::convert::TryFrom;

use super::Bid;
use super::Hand;
use super::Player;
use super::super::Config;

#[derive(Debug)]
pub struct Table {
    pub prize_card: u32,
    pub players: Vec<Player>,
    pub kitty: Hand,
}

impl Table {
    pub fn new() -> Table {
        Table {
            prize_card: 0,
            players: vec![],
            kitty: Hand::new(),
        }
    }
}

fn build_deck(num_cards: u32) -> Vec<u32> {
    let mut deck: Vec<u32> = (1..num_cards+1).collect();
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    deck
}

fn deal_to_table(config: &Config, table: &mut Table) {
    let deck = build_deck(config.num_cards);
    let num_cards_per_hand = usize::try_from(config.num_cards_per_hand).unwrap();
    let hands = deck.chunks(num_cards_per_hand);
    let num_hands = hands.len();
    let mut index = 0;
    for hand in hands {
        let this_hand = Hand{cards: hand.to_vec()};
        if index == 0 {
            table.kitty = this_hand;
        } else {
            let player_index = index - 1;
            table.players[player_index].hand = this_hand;
        }
        // println!("TRACER hello from deal. hand: {:?}", hand);
        index += 1;
    }
    // println!("TRACER hello from deal. deck: {:?}", deck);
}

fn get_bids(prize_card: u32, max_card: u32, players: &mut Vec<Player>) -> Vec<Bid> {
    players.into_iter().map(|p| p.get_bid(prize_card, max_card)).collect()
}

fn determine_winner<'a>(bids: &'a Vec<Bid>) -> &'a Bid<'a> {
    let winning_bid = bids.into_iter().fold(None, |max, bid| match max {
        None => Some(bid),
        Some(y) => Some(if bid.offer > y.offer { bid } else { y }),
    });
    winning_bid.unwrap()
}

fn play_round(table: &mut Table, max_card: u32) -> (u32, String) {
    let prize_card = table.kitty.cards.pop().unwrap();
    println!("\nTRACER play_round prize_card: {}", prize_card);
    let bids = get_bids(prize_card, max_card, &mut table.players);

    for bid in &bids {
        println!("TRACER {}", bid);
    }

    let winning_bid = determine_winner(&bids);
    let winner_name = &winning_bid.bidder.name;
    (prize_card, String::from(winner_name.clone()))
}

fn update_winner(table: &mut Table, prize_card: u32, winner_name: String) {
    for mut player in &mut table.players {
        if player.name == winner_name {
            println!("TRACER {} WINS", winner_name);
            player.wins_round(prize_card);
        }
        println!("TRACER {}", player);
    }
}

fn play_game(config: &Config, table: &mut Table) {
    deal_to_table(config, table);

    println!("TRACER play_game kitty: {}", table.kitty);
    for p in &table.players {
        println!("TRACER play_game {}", p);
    }
    let num_rounds = config.num_cards_per_hand;
    for round_index in 1..(num_rounds+1) {
        let (prize_card, winner_name) = play_round(table, config.num_cards);
        update_winner(table, prize_card, winner_name);
    }
}

pub fn play_tourney(config: &Config, table: &mut Table) {
    for game_index in 0..config.num_games {
        play_game(config, table);
    }
}

mod tests {
    use super::*;

    // some of these tests are enormous, but make me feel more comfortable with the new language

	#[test]
	fn test_determine_winner_basic() {
        let prize_card = 18;
        let p1 = Player{name: String::from("mozart"), .. Player::new()};
        let p2 = Player{name: String::from("beethoven"), .. Player::new()};
        let p3 = Player{name: String::from("liszt"), .. Player::new()};
        let bid1 = Bid{bidder: &p1, offer: 10, prize_card: prize_card};
        let bid2 = Bid{bidder: &p2, offer: 14, prize_card: prize_card};
        let bid3 = Bid{bidder: &p3, offer: 7, prize_card: prize_card};
        let bids = vec![bid1, bid2, bid3];

        // test
        let result = determine_winner(&bids);

		assert_eq!(result.bidder.name, "beethoven");
	}

	#[test]
	fn test_build_deck_basic() {
        let num_cards = 5;

        // test
        let result = build_deck(num_cards);

        let num_cards_result = u32::try_from(result.len()).unwrap();
		assert_eq!(num_cards_result, num_cards);
        for i in 1..num_cards+1 {
            assert!(result.iter().any(|&x| x==i));
        }
	}

	#[test]
	fn test_deal_to_table_basic() {
        let p1 = Player{name: String::from("mozart"), .. Player::new()};
        let p2 = Player{name: String::from("beethoven"), .. Player::new()};
        let players: Vec<Player> = vec![p1, p2];
        let num_players = u32::try_from(players.len()).unwrap();
        let mut table = Table{players: players, .. Table::new()};

        const NUM_GAMES: u32 = 1;
        const NUM_CARDS: u32 = 12;
        let num_cards_per_hand = NUM_CARDS / (num_players + 1);
        let config = Config{num_players: num_players, num_games: NUM_GAMES,
                            num_cards: NUM_CARDS, num_cards_per_hand: num_cards_per_hand};

        // test
        let result = deal_to_table(&config, &mut table);

        let num_cards_kitty = u32::try_from(table.kitty.cards.len()).unwrap();
		assert_eq!(num_cards_kitty, num_cards_per_hand);
        for p in table.players {
            let num_cards_player = u32::try_from(p.hand.cards.len()).unwrap();
            assert_eq!(num_cards_player, num_cards_per_hand);
        }
	}

	#[test]
	fn test_get_bids_basic() {
        let prize_card = 20;
        let max_card = 20;
        let hand1 = Hand{cards: vec![10,11,12]};
        let hand2 = Hand{cards: vec![15,16,17]};
        let p1 = Player{name: String::from("mozart"), hand: hand1, .. Player::new()};
        let p2 = Player{name: String::from("beethoven"), hand: hand2, .. Player::new()};
        let mut players: Vec<Player> = vec![p1, p2];

        // test
        let bids = get_bids(prize_card, max_card, &mut players);

        assert_eq!(2, bids.len());
        let bid1 = &bids[0];
        assert_eq!(10, bid1.offer);
        assert_eq!(20, bid1.prize_card);
        assert_eq!("mozart", bid1.bidder.name);
        assert_eq!(2, bid1.bidder.hand.cards.len());
        let bid2 = &bids[1];
        assert_eq!(15, bid2.offer);
        assert_eq!(20, bid2.prize_card);
        assert_eq!("beethoven", bid2.bidder.name);
        assert_eq!(2, bid2.bidder.hand.cards.len());
    }

	#[test]
	fn test_play_round_basic() {
        let max_card = 12;
        let kitty = Hand{cards: vec![10,11,12]};

        let hand1 = Hand{cards: vec![1,2,3]};
        let hand2 = Hand{cards: vec![4,5,6]};
        let hand3 = Hand{cards: vec![7,8,9]};
        let p1 = Player{name: String::from("mozart"), hand: hand1, .. Player::new()};
        let p2 = Player{name: String::from("beethoven"), hand: hand2, .. Player::new()};
        let p3 = Player{name: String::from("chopin"), hand: hand3, .. Player::new()};
        let players: Vec<Player> = vec![p1, p2, p3];

        let mut table = Table{players: players, kitty: kitty, .. Table::new()};

        // test
        let (prize_card, winner_name) = play_round(&mut table, max_card);

        assert_eq!("chopin", winner_name);
        assert_eq!(12, prize_card);
    }

	#[test]
	fn test_update_winner_basic() {
        let max_card = 12;
        let kitty = Hand{cards: vec![10,11,12]};

        let hand1 = Hand{cards: vec![1,2,3]};
        let hand2 = Hand{cards: vec![4,5,6]};
        let hand3 = Hand{cards: vec![7,8,9]};
        let p1 = Player{name: String::from("mozart"), hand: hand1, .. Player::new()};
        let p2 = Player{name: String::from("beethoven"), hand: hand2, .. Player::new()};
        let p3 = Player{name: String::from("chopin"), hand: hand3, .. Player::new()};
        let players: Vec<Player> = vec![p1, p2, p3];

        let mut table = Table{players: players, kitty: kitty, .. Table::new()};
        let prize_card = 12;
        let winner_name = String::from("chopin");

        // test
        update_winner(&mut table, prize_card, winner_name);

        let winner = &table.players[2];
        assert_eq!(0, winner.player_stats.num_games_won);
        assert_eq!(1, winner.player_stats.num_rounds_won);
        assert_eq!(12, winner.player_stats.total_for_game);
    }
}
