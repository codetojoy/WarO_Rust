
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

fn determine_winner<'a>(prize_card: u32, bids: &'a Vec<Bid>) -> &'a Bid<'a> {
    for bid in bids {
        let player = bid.bidder;
        let name = &player.name;
        let offer = bid.offer;
        println!("TRACER player: {:?} bids {:?} on {:?} and hand: {:?}",
            name, offer, prize_card, player.hand.cards);
    }
    &bids[0]
}

fn play_round(table: &mut Table, max_card: u32) {
    println!("TRACER play_round");
    let prize_card = table.kitty.cards.pop().unwrap();
    println!("TRACER play_round prize_card: {}", prize_card);
    let bids = get_bids(prize_card, max_card, &mut table.players);
    let winning_bid = determine_winner(prize_card, &bids);
}

fn play_game(config: &Config, table: &mut Table) {
    deal_to_table(config, table);

    println!("TRACER play_game kitty: {:?}", table.kitty);
    for p in &table.players {
        println!("TRACER play_game {} : {:?}", p.name, p.hand);
    }
    let num_rounds = config.num_cards_per_hand;
    for round_index in 1..(num_rounds+1) {
        play_round(table, config.num_cards);
    }

    // println!("TRACER play_game table: {:?}", table);
}

pub fn play_tourney(config: &Config, table: &mut Table) {
    println!("TRACER hello from play_tourney");
    for game_index in 0..config.num_games {
        play_game(config, table);
    }
}

mod tests {
    use super::*;

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
}
