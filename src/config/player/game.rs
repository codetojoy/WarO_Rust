
use rand::seq::SliceRandom;
use std::convert::TryFrom;
use std::fmt;

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

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        s.push_str(&format!("\n\nkitty: {}\n", &self.kitty.to_string()));
        s.push_str(&format!("prize_card: {}\n", &self.prize_card.to_string()));
        for player in &self.players {
            s.push_str(&format!("{}\n", &player.to_string()));
        }

        write!(f, "{}", s)
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
    let mut index = 0;

    for hand in hands {
        let this_hand = Hand{cards: hand.to_vec()};
        if index == 0 {
            table.kitty = this_hand;
        } else {
            let player_index = index - 1;
            table.players[player_index].hand = this_hand;
        }
        index += 1;
    }
}

fn get_bids(prize_card: u32, max_card: u32, players: &mut Vec<Player>) -> Vec<Bid> {
    players.into_iter().map(|p| p.get_bid(prize_card, max_card)).collect()
}

fn determine_round_winner<'a>(bids: &'a Vec<Bid>) -> &'a Bid<'a> {
    let winning_bid = bids.into_iter().fold(None, |max, bid| match max {
        None => Some(bid),
        Some(y) => Some(if bid.offer > y.offer { bid } else { y }),
    });
    winning_bid.unwrap()
}

fn update_round_winner(table: &mut Table, prize_card: u32, round_winner_name: String) {
    for player in &mut table.players {
        if player.name == round_winner_name {
            println!("TRACER {} WINS round: ", round_winner_name);
            player.wins_round(prize_card);
        }
        println!("TRACER {}", player);
    }
}

fn play_round(table: &mut Table, max_card: u32) -> (u32, String) {
    let prize_card = table.kitty.cards.pop().unwrap();
    println!("\nTRACER play_round prize_card: {}", prize_card);
    let bids = get_bids(prize_card, max_card, &mut table.players);

    for bid in &bids {
        println!("TRACER {}", bid);
    }

    let winning_bid = determine_round_winner(&bids);
    let winner_name = &winning_bid.bidder.name;
    (prize_card, String::from(winner_name.clone()))
}

fn determine_game_winner<'a>(players: &'a Vec<Player>) -> &'a Player {
    let game_winner = players.into_iter().fold(None, |max, player| match max {
        None => Some(player),
        Some(y) => Some(if player.player_stats.total_for_game > y.player_stats.total_for_game { player } else { y }),
    });
    game_winner.unwrap()
}

fn update_game_winner(table: &mut Table, game_winner_name: String) {
    for player in &mut table.players {
        if player.name == game_winner_name {
            println!("TRACER {} WINS game: ", game_winner_name);
            player.wins_game();
        } else {
            player.loses_game();
        }
        println!("TRACER {}", player);
    }
}

fn play_game(config: &Config, table: &mut Table) -> String {
    deal_to_table(config, table);

    println!("TRACER play_game kitty: {}", table.kitty);
    for p in &table.players {
        println!("TRACER play_game {}", p);
    }
    let num_rounds = config.num_cards_per_hand;
    for _round_index in 1..(num_rounds+1) {
        let (prize_card, round_winner_name) = play_round(table, config.num_cards);
        update_round_winner(table, prize_card, round_winner_name);
    }

    let game_winner = determine_game_winner(&table.players);
    game_winner.name.clone()
}

fn determine_tourney_winner<'a>(players: &'a Vec<Player>) -> &'a Player {
    let tourney_winner = players.into_iter().fold(None, |max, player| match max {
        None => Some(player),
        Some(y) => Some(if player.player_stats.num_games_won > y.player_stats.num_games_won { player } else { y }),
    });
    tourney_winner.unwrap()
}

pub fn play_tourney(config: &Config, table: &mut Table) {
    for _game_index in 0..config.num_games {
        let game_winner_name = play_game(config, table);
        println!("TRACER game {}", game_winner_name);
        update_game_winner(table, game_winner_name);
    }

    let tourney_winner = determine_tourney_winner(&table.players);
    println!("\n\ntourney complete. WINNER: {}", tourney_winner.name);
    println!("final table: {}", table);
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    // some of these tests are enormous, but make me feel more comfortable with the new language

	#[test]
	fn test_determine_round_winner_basic() {
        let prize_card = 18;
        let p1 = Player{name: String::from("mozart"), .. Player::new()};
        let p2 = Player{name: String::from("beethoven"), .. Player::new()};
        let p3 = Player{name: String::from("liszt"), .. Player::new()};
        let bid1 = Bid{bidder: &p1, offer: 10, prize_card: prize_card};
        let bid2 = Bid{bidder: &p2, offer: 14, prize_card: prize_card};
        let bid3 = Bid{bidder: &p3, offer: 7, prize_card: prize_card};
        let bids = vec![bid1, bid2, bid3];

        // test
        let result = determine_round_winner(&bids);

		assert_eq!(result.bidder.name, "beethoven");
	}

	#[test]
	fn test_determine_game_winner_basic() {
        let p1 = Player{name: String::from("mozart"), .. Player::new()};
        let mut p2 = Player{name: String::from("beethoven"), .. Player::new()};
        let p3 = Player{name: String::from("liszt"), .. Player::new()};
        p2.wins_round(10);
        let players = vec![p1, p2, p3];

        // test
        let result = determine_game_winner(&players);

		assert_eq!(result.name, "beethoven");
	}

	#[test]
	fn test_determine_tourney_winner_basic() {
        let p1 = Player{name: String::from("mozart"), .. Player::new()};
        let mut p2 = Player{name: String::from("beethoven"), .. Player::new()};
        let mut p3 = Player{name: String::from("liszt"), .. Player::new()};
        p2.wins_game();
        p3.wins_game();
        p3.wins_game();
        let players = vec![p1, p2, p3];

        // test
        let result = determine_tourney_winner(&players);

		assert_eq!(result.name, "liszt");
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
	fn test_update_round_winner_basic() {
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
        update_round_winner(&mut table, prize_card, winner_name);

        let winner = &table.players[2];
        assert_eq!(0, winner.player_stats.num_games_won);
        assert_eq!(1, winner.player_stats.num_rounds_won);
        assert_eq!(12, winner.player_stats.total_for_game);
    }
}
