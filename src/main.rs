#[macro_use] extern crate rocket;

use std::collections::{HashMap, HashSet};
use std::{env, io};
use std::io::Write;
use std::process;
use std::sync::{Arc, Mutex};
use rand::Rng;
use rand::rngs::OsRng;
use rocket::futures::{SinkExt, StreamExt};
use rocket::futures::stream::{SplitSink, SplitStream};
use serde_json::{from_str, json};

mod utils;
// mod hand;
pub mod game;
mod messages;

const MAX_PLAYERS: i32 = 50;
const MAX_PLAYERS_PER_GAME: i32 = 10;
const MESSAGE_SEND_ERROR: &'static str = "Error sending message";
const MESSAGE_READ_ERROR: &'static str = "Error reading message";
const SERIALIZATION_ERROR: &'static str = "Error serializing message";
const DESERIALIZATION_ERROR: &'static str = "Error deserializing message";

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

use rocket::http::Status;
use rocket::response::status;
use rocket::tokio::net::TcpListener;
use rocket::tokio::task;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use crate::game::Game;
use crate::messages::{Handshake, HandshakeOk, StartNewTable, StartNewTableOk};

#[get("/ws")]
async fn websocket_handler() -> &'static str {
    "WebSocket server is running"
}

async fn handle_connection(
    stream: TcpStream,
    mut client_ids: Arc<Mutex<HashSet<u128>>>,
    mut game_ids: Arc<Mutex<HashMap<u128, Game>>>,
    mut rng: Arc<Mutex<OsRng>>
) {

    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake");

    // let (mut write, mut read) = ws_stream.split();
    let (mut write, mut read): (SplitSink<WebSocketStream<TcpStream>, Message>, SplitStream<WebSocketStream<TcpStream>>) = ws_stream.split();

    let client_id: u128 = client_handshake(&mut client_ids, &mut rng, &mut write, &mut read).await;

    let Some(msg) = read.next().await else {todo!()};
    let msg = msg.expect(MESSAGE_READ_ERROR);

    // Deserialize JSON message into StartNewTable struct
    if let Ok(start_new_table) = from_str::<StartNewTable>(msg.to_text().expect("Failed to convert to text")) {
        println!("Received StartNewTable: {:?}", start_new_table);

        let game_id: u128 = utils::get_unique_game_id(&game_ids, rng).await;

        println!("Client {} joined table {}. Game created.", client_id, game_id);
        let message = StartNewTableOk::new(client_id, game_id);
        let json_message = serde_json::to_string(&message).expect(SERIALIZATION_ERROR);
        send_message(&mut write, json_message).await;

    } else {
        panic!("{}", DESERIALIZATION_ERROR);
    }

    dbg!(client_ids);
    dbg!(game_ids);
}


async fn client_handshake<'a>(mut client_ids: &'a Arc<Mutex<HashSet<u128>>>, mut rng: &'a Arc<Mutex<OsRng>>, write: &'a mut SplitSink<WebSocketStream<TcpStream>, Message>, read: &'a mut SplitStream<WebSocketStream<TcpStream>>) -> u128 {

    let Some(msg) = read.next().await else {todo!()};
    let msg = msg.expect(MESSAGE_READ_ERROR);
    let client_id: u128 = 0;

    // Deserialize JSON message into Handshake struct
    if let Ok(handshake_ok) = from_str::<Handshake>(msg.to_text().expect("Failed to convert to text")) {
        println!("Received Handshake: {:?}", handshake_ok);

        let client_id = utils::get_unique_client_id(&client_ids, rng).await;
        println!("Created client id: {}", client_id);
    } else {
        dbg!(msg);
        panic!("{}", DESERIALIZATION_ERROR);
    }
    
    let message = HandshakeOk::new(client_id);
    let json_message = serde_json::to_string(&message).expect(SERIALIZATION_ERROR);

    send_message(write, json_message).await;
    return client_id;
}

async fn send_message(mut write: &mut SplitSink<WebSocketStream<TcpStream>, Message>, json_message: String) {
    // let json_message = serde_json::to_string(&message).expect(SERIALIZATION_ERROR);
    write.send(Message::Text(json_message)).await.expect(MESSAGE_SEND_ERROR);
}

#[rocket::main]
async fn main()  {
    let addr = "127.0.0.1:9001".to_string();

    let rng: Arc<Mutex<OsRng>> = Arc::new(Mutex::new(OsRng::default()));
    let client_ids: Arc<Mutex<HashSet<u128>>> = Arc::new(Mutex::new(HashSet::<u128>::new()));
    let game_ids: Arc<Mutex<HashMap<u128, Game>>> = Arc::new(Mutex::new(HashMap::<u128, Game>::new()));

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    task::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let rng_clone = Arc::clone(&rng);
            let client_ids_clone = Arc::clone(&client_ids);
            let game_ids_clone = Arc::clone(&game_ids);
            task::spawn(handle_connection(stream, client_ids_clone, game_ids_clone, rng_clone));
        }
        });

    println!("WebSocket server started");

    rocket::build()
        .mount("/", routes![websocket_handler])
        .launch()
        .await
        .unwrap();
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let debug: bool = args.get(1).unwrap_or(&"NOT DEBUG".to_string()) == "--DEBUG";
//
//     // test1();
//     // test2();
//     // test3();
//     // local_game_test(false, debug);
// }

// fn test1() {
//     let mut game_ids = HashSet::new();
//     let mut player_ids = HashSet::new();
//     let mut input = String::new();

//     println!("Tell me when to start!");
//     io::stdin().read_line(&mut input).unwrap();

//     if input.trim() == "start" {
//         let game_id = utils::get_unique_id(&game_ids);
//         game_ids.insert(game_id);
//         println!("Game id: {}", game_id);
//     } else {
//         process::exit(1);
//     }

//     println!("how many players are playing? ");
//     input.clear();
//     io::stdin().read_line(&mut input).unwrap();

//     let num_players = input.trim().parse::<i32>().unwrap();
//     let game_id = utils::get_unique_id(&game_ids);
//     let mut game = types::Game::new(game_id);

//     for _ in 0..num_players {
//         let player_id = utils::get_unique_id(&player_ids);
//         player_ids.insert(player_id);
//         println!("Player id: {}", player_id);
//         game.add_player(types::Player::new(player_id, "Player".to_string(), 1000));
//     }

//     println!("Number of players: {}\n", game.get_num_players());
//     println!("Game id: {}", game.get_game_id());
//     println!("Game players: {}", game.get_num_players());
//     println!();
//     println!("What size chips do you want to use?: ");
//     input.clear();
//     io::stdin().read_line(&mut input).unwrap();

//     let split: Vec<&str> = input.trim().split(" ").collect();
//     let mut chip_sizes = Vec::<i8>::new();

//     println!("num inputs: {}", split.len());
//     if split.len() != 4 {
//         println!("Please enter four numbers");
//         process::exit(1);
//     }

//     for thing in split.iter() {
//         let size = thing.parse::<i8>().unwrap();
//         chip_sizes.push(size);
//     }
// }


// fn test2() {

//     let values1 = vec![3, 3, 3, 2, 2];
//     let suit1 = vec![hand::Suit::Clubs, hand::Suit::Diamonds, hand::Suit::Hearts, hand::Suit::Clubs, hand::Suit::Diamonds];

//     let values2 = vec![2, 2, 2, 14, 14];
//     let suit2 = vec![hand::Suit::Clubs, hand::Suit::Diamonds, hand::Suit::Hearts, hand::Suit::Clubs, hand::Suit::Diamonds];

//     let mut cards1 = Vec::<hand::Card>::new();
//     let mut cards2 = Vec::<hand::Card>::new();

//     for (suit, value) in suit1.iter().zip(values1.iter()) {
//         cards1.push(hand::Card::new(*suit, *value));
//     }

//     for (suit, value) in suit2.iter().zip(values2.iter()) {
//         cards2.push(hand::Card::new(*suit, *value));
//     }

//     let mut my_cards1 = HashSet::<hand::Card>::new();
//     let mut my_cards2 = HashSet::<hand::Card>::new();

//     for card in cards1 {
//         my_cards1.insert(card);
//     }

//     for card in cards2 {
//         my_cards2.insert(card);
//     }

//     let hand1 = hand::Hand::new(my_cards1);
//     let hand2 = hand::Hand::new(my_cards2);

//     for card in hand1.get_cards() {
//         println!("{}", card);
//     }
//     println!();
//     for card in hand2.get_cards() {
//         println!("{}", card);
//     }
//     println!();
//     println!("OUTPUTS:");
//     let hand1_score = hand1.check_hand();
//     let hand2_score = hand2.check_hand();

//     println!("{}", hand1_score);
//     println!("{}", hand2_score);

//     if hand1_score > hand2_score {
//         println!("Player 1 wins!");
//         println!("With a hand of {}", hand1_score.get_hand_type());
//     } else if hand1_score < hand2_score {
//         println!("Player 2 wins!");
//         println!("With a hand of {}", hand2_score.get_hand_type());
//     } else {
//         println!("It's a tie!");
//         println!("With a hand of {}", hand1_score.get_hand_type());
//     }
    
// }


// fn test3() {
//     let community_cards = vec![
//         hand::Card::new(hand::Suit::Spades, 3),
//         hand::Card::new(hand::Suit::Diamonds, 3),
//         hand::Card::new(hand::Suit::Clubs, 3),
//         hand::Card::new(hand::Suit::Diamonds, 2),
//         hand::Card::new(hand::Suit::Spades, 2)
//     ];

//     let player1_hole_cards = vec![
//         hand::Card::new(hand::Suit::Hearts, 11),
//         hand::Card::new(hand::Suit::Clubs, 3)
//     ];

//     let player2_hole_cards = vec![
//         hand::Card::new(hand::Suit::Diamonds, 11),
//         hand::Card::new(hand::Suit::Hearts, 6)
//     ];

//     let all_possible1 = community_cards.iter().chain(player1_hole_cards.iter()).cloned().collect();
//     let all_possible2 = community_cards.iter().chain(player2_hole_cards.iter()).cloned().collect();

//     let hand1 = hand::OnePlayerAllPossibleCards::new(all_possible1);
//     let hand2 = hand::OnePlayerAllPossibleCards::new(all_possible2);

//     let hand1_score = hand1.get_highest_hand_score();
//     let hand2_score = hand2.get_highest_hand_score();

//     println!("Player 1 hand: {}", hand1_score.get_hand_type());
//     println!("Player 2 hand: {}", hand2_score.get_hand_type());

//     if hand1_score > hand2_score {
//         println!("Player 1 wins!");
//     } else if hand1_score < hand2_score {
//         println!("Player 2 wins!");
//     } else {
//         println!("It's a tie!");
//     }
// }


// fn local_game_test (ask_for_chip_sizes: bool, debug: bool) {
//     let mut game_ids = HashSet::new();
//     let mut player_ids = HashSet::new();
//     let mut input = String::new();
//
//     let game_id = utils::get_unique_id(&game_ids);
//     game_ids.insert(game_id);
//     if debug {
//         println!("Game id: {}", game_id);
//     }
//
//     print!("How many players are playing? ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input).unwrap();
//
//     let num_players = input.trim().parse::<i32>().unwrap();
//     print!("\n");
//
//     if num_players < 2 {
//         println!("You need at least 2 players to play");
//         process::exit(1);
//     }
//
//     if num_players > MAX_PLAYERS_PER_GAME {
//         println!("You can't have more than {} players", MAX_PLAYERS_PER_GAME);
//         process::exit(1);
//     }
//
//     let game_id = utils::get_unique_id(&game_ids);
//     let mut game = game::Game::new(game_id, 2, 1000);
//
//     for _ in 0..num_players {
//         let player_id = utils::get_unique_id(&player_ids);
//         player_ids.insert(player_id);
//         let player_name = format!("Player#{}", player_id);
//         game.add_player(game::player::Player::new(player_id, player_name, 1000));
//     }
//
//     if debug {
//         println!("Number of players: {}", game.get_num_players());
//         println!("Game id: {}", game.get_game_id());
//         println!("Game players: {}", game.get_num_players());
//         println!();
//     }
//     let mut chip_sizes: Vec::<i8>;
//
//     if ask_for_chip_sizes {
//         println!("What size chips do you want to use?: ");
//         input.clear();
//         io::stdin().read_line(&mut input).unwrap();
//
//         let split: Vec<&str> = input.trim().split(" ").collect();
//         chip_sizes = Vec::<i8>::new();
//
//         println!("num inputs: {}", split.len());
//         if split.len() != 4 {
//             println!("Please enter four numbers");
//             process::exit(1);
//         }
//
//         for thing in split.iter() {
//             let size = thing.parse::<i8>().unwrap();
//             chip_sizes.push(size);
//         }
//     } else {
//         chip_sizes = vec![10, 25, 50, 100];
//     }
//
//     if debug {
//         println!("Chip sizes: {:?}", chip_sizes);
//     }
//
//     game.start_game(debug);
//
// }
