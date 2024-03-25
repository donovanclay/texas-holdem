// use std::collections::HashMap;
use std::collections::HashSet;
// use queue::Queue;

use std::io;
use std::process;
// use std::str::Split;

mod types;
mod utils;

const MAX_PLAYERS: i32 = 50;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn test1() {
    let mut game_ids = HashSet::new();
    let mut player_ids = HashSet::new();
    let mut input = String::new();

    println!("Tell me when to start!");
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "start" {
        let game_id = utils::get_unique_id(&game_ids);
        game_ids.insert(game_id);
        println!("Game id: {}", game_id);
    } else {
        process::exit(1);
    }

    println!("how many players are playing? ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let num_players = input.trim().parse::<i32>().unwrap();
    let game_id = utils::get_unique_id(&game_ids);
    let mut game = types::Game::new(game_id);

    for _ in 0..num_players {
        let player_id = utils::get_unique_id(&player_ids);
        player_ids.insert(player_id);
        println!("Player id: {}", player_id);
        game.add_player(types::Player::new(player_id, "Player".to_string(), 1000));
    }

    println!("Number of players: {}\n", game.get_num_players());
    println!("Game id: {}", game.get_game_id());
    println!("Game players: {}", game.get_num_players());
    println!();
    println!("What size chips do you want to use?: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let split: Vec<&str> = input.trim().split(" ").collect();
    let mut chip_sizes = Vec::<i8>::new();

    println!("num inputs: {}", split.len());
    if split.len() != 4 {
        println!("Please enter four numbers");
        process::exit(1);
    }

    for thing in split.iter() {
        let size = thing.parse::<i8>().unwrap();
        chip_sizes.push(size);
    }
}


fn test2() {

    let values1 = vec![14, 10, 11, 12, 13];
    let suit1 = vec![types::Suit::Clubs, types::Suit::Clubs, types::Suit::Clubs, types::Suit::Clubs, types::Suit::Clubs];

    let values2 = vec![14, 10, 11, 12, 13];
    let suit2 = vec![types::Suit::Clubs, types::Suit::Clubs, types::Suit::Clubs, types::Suit::Clubs, types::Suit::Diamonds];

    let mut cards1 = Vec::<types::Card>::new();
    let mut cards2 = Vec::<types::Card>::new();

    for (suit, value) in suit1.iter().zip(values1.iter()) {
        cards1.push(types::Card::new(*suit, *value));
    }

    for (suit, value) in suit2.iter().zip(values2.iter()) {
        cards2.push(types::Card::new(*suit, *value));
    }

    let mut my_cards1 = HashSet::<types::Card>::new();
    let mut my_cards2 = HashSet::<types::Card>::new();

    for card in cards1 {
        my_cards1.insert(card);
    }

    for card in cards2 {
        my_cards2.insert(card);
    }

    let hand1 = types::Hand::new(my_cards1);
    let hand2 = types::Hand::new(my_cards2);

    for card in hand1.get_cards() {
        println!("{}", card);
    }
    println!();
    for card in hand2.get_cards() {
        println!("{}", card);
    }
    println!();
    println!("OUTPUTS:");
    let hand1_score = hand1.check_hand();
    let hand2_score = hand2.check_hand();

    println!("{}", hand1_score);
    println!("{}", hand2_score);

    if hand1_score > hand2_score {
        println!("Player 1 wins!");
    } else if hand1_score < hand2_score {
        println!("Player 2 wins!");
    } else {
        println!("It's a tie!");
    }
}

fn main() {
    // test1();
    test2();
}
