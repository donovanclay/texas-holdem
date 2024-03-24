// use std::collections::HashMap;
use std::collections::HashSet;
use queue::Queue;

use std::io;
use std::process;
use std::str::Split;

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

    let mut num_players = input.trim().parse::<i32>().unwrap();
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

    let mut split: Vec<&str> = input.trim().split(" ").collect();
    let mut chip_sizes = Vec::<i8>::new();

    println!("num inputs: {}", split.len());
    if split.len() != 4 {
        println!("Please enter four numbers");
        process::exit(1);
    }

    for thing in split.iter() {
        // println!("{}", thing);
        let size = thing.parse::<i8>().unwrap();
        chip_sizes.push(size);
    }
}


fn test2() {
    let suit1 = types::Suit::Hearts;
    let suit2 = types::Suit::Clubs;
    let suit3 = types::Suit::Diamonds;
    let suit4 = types::Suit::Hearts;
    let suit5 = types::Suit::Spades;

    let card1 = types::Card::new(types::Suit::Hearts, 1);
    let card2 = types::Card::new(types::Suit::Diamonds, 12);
    let card3 = types::Card::new(types::Suit::Hearts, 13);
    let card4 = types::Card::new(types::Suit::Spades, 10);
    let card5 = types::Card::new(types::Suit::Hearts, 11);

    let mut hand = types::Hand::new();
    hand.add_card(card1);
    hand.add_card(card2);
    hand.add_card(card3);
    hand.add_card(card4);
    hand.add_card(card5);

    for card in hand.get_cards() {
        println!("{}", card);
    }

    let pairs = hand.check_pair();
    let two_pairs = hand.check_two_pair();
    let triplets = hand.check_three_of_a_kind();
    let quads = hand.check_four_of_a_kind();

    match pairs {
        Some(pairs) => {
            println!("Pairs: ");
            for pair in pairs {
                println!("Pair: ");
                for card in pair {
                    println!("{}", card);
                }
            }
        },
        None => {
            println!("No pairs");
        }
    }

    match two_pairs {
        Some(two_pairs) => {
            println!("Two pairs: ");
            for two_pair in two_pairs {
                println!("Two pair: ");
                for card in two_pair {
                    println!("{}", card);
                }
            }
        },
        None => {
            println!("No two pairs");
        }
    }

    match triplets {
        Some(triplets) => {
            println!("Triplets: ");
            for triple in triplets {
                println!("Triple: ");
                for card in triple {
                    println!("{}", card);
                }
            }
        },
        None => {
            println!("No triplets");
        }
    }

    match quads {
        Some(quads) => {
            println!("Quads: ");
            for quad in quads {
                println!("Quad: ");
                for card in quad {
                    println!("{}", card);
                }
            }
        },
        None => {
            println!("No quads");
        }
    }

    let full_houses = hand.check_full_house();

    println!("Number of full houses: {}", full_houses.len());
    for full_house in full_houses {
        for card in full_house {
            println!("{}", card);
        }
    }

    println!("Flush: {}", hand.check_flush());
    println!("Royal: {}", hand.check_royal());
    println!("Straight: {}", hand.check_straight());

    println!("OUTPUTS:");
    hand.check_hand();
}

fn main() {
    // test1();
    test2();



    // print_type_of(&split);

    // for thing in split {
    //     println!("{}", thing);
    // }
}
