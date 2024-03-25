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

    let card1 = types::Card::new(types::Suit::Hearts, 13);
    let card2 = types::Card::new(types::Suit::Clubs, 14);
    let card3 = types::Card::new(types::Suit::Spades, 12);
    let card4 = types::Card::new(types::Suit::Clubs, 12);
    let card5 = types::Card::new(types::Suit::Hearts, 12);

    let mut my_cards = HashSet::<types::Card>::new();
    my_cards.insert(card1);
    my_cards.insert(card2);
    my_cards.insert(card3);
    my_cards.insert(card4);
    my_cards.insert(card5);

    let mut hand = types::Hand::new(my_cards);

    for card in hand.get_cards() {
        println!("{}", card);
    }
    println!();

    let high_card_output = hand.check_high_card();
    let pairs_output= hand.check_pair();
    let two_pairs_output = hand.check_two_pair();
    let triplets_output = hand.check_three_of_a_kind();
    let quads_output = hand.check_four_of_a_kind();
    let full_houses_output = hand.check_full_house();

    let functions = vec![types::Hand::check_high_card, types::Hand::check_pair, types::Hand::check_two_pair, types::Hand::check_three_of_a_kind, types::Hand::check_four_of_a_kind, types::Hand::check_full_house];
    let output_types = vec!["High Card", "Pairs", "Two Pairs", "Triplets", "Quads", "Full Houses"];

    for (index, func) in functions.iter().enumerate() {
        let output: Option<(HashSet<Vec<types::Card>>, i32, Vec<types::Card>)> = func(&hand);

        match output {
            Some((cards, highest_value, highest_of_this_type)) => {
                println!("Highest Value {}: {}", output_types[index], highest_value);
                for card in highest_of_this_type {
                    println!("{}", card);
                }
                println!();
            },
            None => {
                println!("No {}", output_types[index])
            }
        }
    }

    // {
    //     for output in outputs.iter() {
    //         match output {
    //             Some((cards, value, _)) => {
    //                 println!("Value: {}", value);
    //                 for card in cards {
    //                     println!("{}", card);
    //                 }
    //                 println!();
    //             },
    //             None => {
    //                 println!("No pairs");
    //             }
    //         }
    //     }

    //     match pairs_output{
    //         Some((pairs, highest_value, _)) => {
    //             println!("Highest value pair: {}", highest_value);
    //         },
    //         None => {
    //             println!("No pairs");
    //         }
    //     }

    //     match two_pairs_output {
    //         Some((two_pairs, highest_value, _)) => {
    //             println!("Highest value two pair: {}", highest_value);
    //         },
    //         None => {
    //             println!("No two pairs");
    //         }
    //     }

    //     match triplets_output {
    //         Some((triplets, highest_value, highest_triplet)) => {
    //             println!("Highest value triplet: {}", highest_value);

    //             println!("Highest triplet:")

    //         },
    //         None => {
    //             println!("No triplets");
    //         }
    //     }

    //     match quads_output {
    //         Some((quads, highest_value, highest_quad)) => {
    //             println!("Highest value quad: {}", highest_value);

    //             println!("Highest quad:");
    //             for card in highest_quad {
    //                 println!("{}", card);
    //             }
    //             println!();
    //         },
    //         None => {
    //             println!("No quads");
    //         }
    //     }

    //     match full_houses_output {
    //         Some((full_houses, highest_value, highest_full_house)) => {
    //             println!("Highest value full house: {}", highest_value);
                
    //             println!("Highest full house:");
    //             for card in highest_full_house {
    //                 println!("{}", card);
    //             }
    //             println!();
    //         },
    //         None => {
    //             println!("No full houses");
    //         }
    //     }
    // }
    

    println!("Flush: {}", hand.check_flush());
    println!("Royal: {}", hand.check_royal());
    println!("Straight: {}", hand.check_straight());
    println!();
    println!("OUTPUTS:");
    hand.check_hand();
}

fn main() {
    // test1();
    test2();
}
