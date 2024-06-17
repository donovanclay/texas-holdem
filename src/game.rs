use std::cmp::max;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::collections::HashMap;
use std::io;
use std::io::Write;

use colored::Colorize;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;

use crate::game::hand::{Card, OnePlayerAllPossibleCards};
use crate::game::player::{Player, PlayerId};
use crate::utils;

pub mod player;
pub mod hand;


const PLAYER_NOT_FOUND_ERROR: &str = "Expected another player in the round. There was none.";


/// Represents a game of poker.
///
/// # Fields
///
/// * `game_id`: An `i32` that uniquely identifies the game.
/// * `num_players`: An `i32` that represents the number of players in the game.
/// * `players`: A `VecDeque` of `PlayerId`s that represents the order of players in the game.
/// * `players_in_round`: A `VecDeque` of `PlayerId`s that represents the order of players in the current round.
/// * `player_id_to_player`: A `HashMap` that maps `PlayerId`s to `Player`s.
/// * `big_blind`: An `i32` that represents the big blind amount.
/// * `dealer_location`: An `i16` that represents the index of the dealer in the `players` `VecDeque`.
/// * `community_cards`: A `Vec` of `hand::Card`s that represents the community cards.
/// * `curr_bet`: An `i32` that represents the current bet amount.
/// * `last_player_to_raise`: A `PlayerId` that represents the last player to raise.
/// * `bet_this_round`: A `HashSet` of `PlayerId`s that represents the players who have bet in this round.
/// * `has_raised`: A `bool` that indicates whether a player has raised in the current round.
/// * `bets`: A `HashMap` that maps `PlayerId`s to the amount they have bet.
/// * `pots`: A `BTreeMap` that maps from a bet amount to the players that have bet that amount.
#[derive(Debug)]
pub struct Game {
    game_id: u128,
    num_players: i32,
    players: VecDeque<PlayerId>,
    players_in_round: HashSet<PlayerId>,
    turn_queue: VecDeque<PlayerId>,
    player_id_to_player: HashMap<PlayerId, Player>,
    big_blind: i32,
    initial_money: i32,
    dealer_location: i16,
    community_cards: Vec<hand::Card>,
    curr_bet: i32,
    last_player_to_raise: PlayerId,
    bet_this_round: HashSet<PlayerId>,
    has_raised: bool,
    bets: HashMap<PlayerId, i32>,
    /// A `BTreeMap` that maps from a bet amount to the players that have bet that amount.
    pots: BTreeMap<i32, HashSet<PlayerId>>,
}


impl Game {
    pub fn new(game_id: u128, big_blind: i32, initial_money: i32) -> Game {
        Game {
            game_id,
            num_players: 0,
            players: VecDeque::new(),
            players_in_round: HashSet::new(),
            turn_queue: VecDeque::new(),
            player_id_to_player: HashMap::new(),
            big_blind,
            initial_money,
            dealer_location: 0,
            community_cards: Vec::<hand::Card>::new(),
            curr_bet: big_blind,
            last_player_to_raise: 0,
            bet_this_round: HashSet::new(),
            has_raised: false,
            bets: HashMap::new(),
            pots: BTreeMap::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push_back(player.get_player_id());
        self.player_id_to_player.insert(player.get_player_id(), player.clone());
        self.num_players += 1;
    }


    pub fn get_num_players(&self) -> i32 {
        self.num_players
    }


    pub fn get_game_id(&self) -> u128 {
        self.game_id
    }


    pub fn get_big_blind(&self) -> i32 {
        self.big_blind
    }


    fn print_community_cards(&self) {
        println!("Community Cards:");
        if self.community_cards.len() != 0 {
            for card in &self.community_cards {
                println!("{}", card);
            }
        } else {
            println!("Community cards have not been dealt.")
        }

    }

    fn format_community_cards(&self) -> VecDeque<String> {
        let mut output = VecDeque::<String>::new();
        if self.community_cards.len() != 0 {
            for card in &self.community_cards {
                output.push_back(card.to_string());
            }
        } else {
            output.push_back("Community cards have not been dealt.".to_string());
        }

        output
    }


    /// Prints the current turn state of the game.
    ///
    /// # Notes
    ///
    /// This function prints the current turn state of the game in a formatted manner. It displays the players in the current round,
    /// with arrows pointing to the player whose turn it is. The players are displayed in the order they will play their turns.
    ///
    /// This function does not return any value and does not modify the game state.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no players in the current round.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `game` is an instance of the `Game` struct
    /// game.print_turn_state();
    /// ```
    fn print_turn_state(&self) {
        if self.turn_queue.len() == 0 {
            return;
        }

        let players = self.turn_queue.clone();
        let width =  players.front().expect(PLAYER_NOT_FOUND_ERROR).to_string().len() / 2
            + "Player: ".len();

        let mut output = VecDeque::<VecDeque<String>>::new();

        players.iter().enumerate()
            .for_each(|(i, player)| {
                let player_str = player.to_string();
                let player_contribution = self.bets.get(player).unwrap_or(&0).to_string();
                let player_money = self.player_id_to_player.get(player).expect(PLAYER_NOT_FOUND_ERROR).get_money().to_string();

                let mut this_output = VecDeque::<String>::new();

                if i == 0 {
                    this_output.push_back(" ".repeat(width) + "|");
                    this_output.push_back(" ".repeat(width) + "|");
                    this_output.push_back(" ".repeat(width) + "v");
                    // this_output.push_back(" ".repeat(width) + "a");
                    // this_output.push_back("Next to go.".to_string());
                } else {
                    for _ in 0..3 {
                        this_output.push_back("".to_string());
                    }
                }

                this_output.push_back("Player: ".to_string() + &*player_str);
                this_output.push_back("Contribution: ".to_string() + &*player_contribution);
                this_output.push_back("Money: ".to_string() + &*player_money);

                output.push_back(this_output);
            });
            // .collect::<Vec<_>>()
            // .join(", ");

        // let turn_state = " Turn State ";
        //
        // let dashes = "-".repeat((max((players_str.len() as i32) - (turn_state.len() as i32), 0) / 2) as usize);
        //
        // println!();
        // println!("{}", dashes.clone() + turn_state + &dashes);
        // println!("{}", " ".repeat(width) + "|");
        // println!("{}", " ".repeat(width) + "|");
        // println!("{}", " ".repeat(width) + "▼");
        // println!("{}", players_str);
        // println!();

        let output_str = utils::format_next_to_each_other(output);

        let turn_state = " Turn State ";

        let num_dashes_total = output_str.split("\n").collect::<Vec<&str>>().iter().map(|str| str.len()).max().expect("Error in calculating dashes");

        let dashes = "-".repeat((max((num_dashes_total as i32) - (turn_state.len() as i32), 0) / 2) as usize);

        println!();
        println!("{}", dashes.clone() + turn_state + &dashes);
        println!("{}", output_str);

        // println!("------------------ Turn State ------------------\n{}", output_str);
    }

    /// Starts the game and handles the main game loop.
    ///
    /// # Parameters
    ///
    /// * `debug`: A `bool` indicating whether to print debug information.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no players in the game, or if a player tries to raise by an amount less than the current bet.
    ///
    /// # Notes
    ///
    /// This function starts the game, deals the hole cards, the flop, the turn, and the river, determines the winner, and rotates the dealer's location. It also clears the bets at the start of the game. If `debug` is `true`, it prints a message indicating that the game has started.
    ///
    /// This function does not return any value.
    pub fn start_game(&mut self, debug: bool) {
        if debug {
            println!("Game {} has started!", self.game_id);
        }

        let max_round = 5;
        let mut round = 1;

        while self.players.len() > 1 && round < max_round {
            println!("Starting round #{}", round);

            self.play_one_round(debug);
            // assert_eq!(self.get_total_player_money(), self.players.len() as i32 * self.initial_money, "Incorrect amount of money in the game");

            // rotate the dealers and players queue
            self.players.rotate_left(1);

            self.dealer_location = (self.dealer_location + 1) % self.players.len() as i16;
            round += 1;

            // self.players_in_round.clear();



            // for player_id in self.players.iter() {
            //     let player = self.player_id_to_player.get(player_id).expect(PLAYER_NOT_FOUND_ERROR);
            //     if player.get_money() > 0 {
            //         self.turn_queue.push_back(*player_id);
            //     }
            // }

            if self.turn_queue.len() == 1 {
                println!("There is a winner");
                for player_id in self.players.iter() {
                    let player = self.player_id_to_player.get(player_id).expect(PLAYER_NOT_FOUND_ERROR);

                    dbg!(player);
                }
                return;
            }


        }
    }

    fn play_one_round(&mut self, debug: bool) {

        // // Initialize the pots map.
        // self.pots.insert(BTreeSet::from_iter(self.players.iter().cloned().collect::<Vec<_>>()), 0);

        let mut deck = hand::Card::new_full_deck();

        if !self.deal_hole_cards(&mut deck, debug) { self.determine_winner(); self.clear_round_data(); return }

        if !self.deal_flop(&mut deck, debug) { self.determine_winner(); self.clear_round_data(); return }

        // Deal the turn.
        if !self.deal_single_card(&mut deck, debug) { self.determine_winner(); self.clear_round_data(); return }

        // Deal the river.
        if !self.deal_single_card(&mut deck, debug) { self.determine_winner(); self.clear_round_data(); return }

        self.determine_winner();
        self.clear_round_data();
    }

    fn clear_round_data(&mut self) {
        self.pots.clear();
        self.community_cards.clear();
        self.players_in_round.clear();
        self.turn_queue.clear();
        self.curr_bet = 0;
        self.bet_this_round.clear();
        self.bets.clear();
    }

    /// Deals two hole cards to each player from the deck and initiates the first round of betting.
    ///
    /// # Arguments
    ///
    /// * `deck` - A mutable reference to a `HashSet` of `hand::Card` representing the deck of cards.
    /// * `debug` - A `bool` indicating whether debug information should be printed to the console.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if a winner is determined during the first round of betting, otherwise returns `false`.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no players in the game, or if the deck runs out of cards.
    ///
    /// # Notes
    ///
    /// This function initializes a full deck of cards, deals two hole cards to each player, and handles the small and big blinds. It then calls `circle_players` to rotate through the players in the current round, prompting each to make a decision. After `circle_players` returns, it resets the current bet to 0.
    ///
    /// If `debug` is `true`, this function also prints debug information, such as the size of the deck, the number of players in the current round, the size of the blinds, and the hole cards of each player.
    fn deal_hole_cards(&mut self, deck: &mut HashSet<hand::Card>, debug: bool) -> bool {

        if debug {
            println!("Dealing hole cards.");
        }

        // Initialize the previous contributions map.
        let mut prev_contributions = HashMap::<PlayerId, i32>::new();

        // deal the hole cards to each player
        // for _ in 0..self.players.len() {
        for &player_id in self.players.iter() {
            // let player_id = self.players.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
            let mut player = self.player_id_to_player.get_mut(&player_id).expect(PLAYER_NOT_FOUND_ERROR);

            if (player.get_money() == 0) {
                continue
            }

            let mut hole_cards = Vec::<Card>::new();
            for _ in 0..2 {
                let card = deck.iter().choose(&mut rand::thread_rng()).expect("Deck ran out of cards").clone();
                hole_cards.push(card);
                deck.remove(&card);
            }

            player.set_hole_cards(hole_cards.clone());
            self.turn_queue.push_back(player_id);
            self.players_in_round.insert(player_id);
        }

        let num_players_in_round = self.turn_queue.len() as i32;

        if debug {
            println!("Size of deck: {}", deck.len());
            println!("Number of players this round: {}", num_players_in_round);
            println!("Size of blinds: {}, {}", self.big_blind, self.big_blind / 2);
        }

        // have the small blind and big blind pay
        let mut prev_player: PlayerId;

        let player_id = self.turn_queue.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
        prev_player = player_id;


        self.make_player_bet(player_id, self.big_blind / 2, 0);
        prev_contributions.insert(player_id, self.big_blind / 2);

        self.last_player_to_raise = player_id;
        self.turn_queue.push_back(player_id);

        let player_id = self.turn_queue.pop_front().expect(PLAYER_NOT_FOUND_ERROR);

        prev_player = player_id;

        self.make_player_bet(player_id, self.big_blind, 0);
        prev_contributions.insert(player_id, self.big_blind);

        self.last_player_to_raise = player_id;
        self.turn_queue.push_back(player_id);


        self.has_raised = false;
        let has_winner: bool = self.circle_players(&mut Some(prev_contributions), &mut Some(prev_player), true);

        self.curr_bet = 0;

        has_winner
    }


    /// Deals the flop (the first three community cards) in the game.
    ///
    /// # Parameters
    ///
    /// * `deck`: A mutable reference to a `HashSet` of `hand::Card`s representing the current deck.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if a winner is determined during the second round of betting, otherwise returns `false`.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no cards left in the deck.
    ///
    /// # Notes
    ///
    /// This function randomly selects three cards from the deck, removes them from the deck, and adds them to the community cards. It then prints the community cards and the size of the deck. After that, it calls `circle_players` to rotate through the players in the current round, prompting each to make a decision. After `circle_players` returns, it resets the current bet to 0.
    fn deal_flop(&mut self, deck: &mut HashSet<hand::Card>, debug: bool) -> bool {
        if debug {
            println!("Dealing flop.");
        }

        let mut community_cards = Vec::<hand::Card>::new();

        for _ in 0..3 {
            let card = deck.iter().choose(&mut rand::thread_rng()).expect("Deck ran out of cards").clone();
            community_cards.push(card);
            deck.remove(&card);
        }

        self.community_cards = community_cards.clone();

        let has_winner: bool = self.circle_players(&mut None, &mut None, false);

        self.curr_bet = 0;

        has_winner
    }


    /// Deals a single card to the community cards in the game.
    ///
    /// # Parameters
    ///
    /// * `deck`: A mutable reference to a `HashSet` of `hand::Card`s representing the current deck.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if a winner is determined during the third round of betting, otherwise returns `false`.
    ///
    ///
    /// # Panics
    ///
    /// This function will panic if there are no cards left in the deck.
    ///
    /// # Notes
    ///
    /// This function randomly selects a card from the deck, removes it from the deck, and adds it to the community cards. It then prints the community cards and the size of the deck. After that, it calls `circle_players` to rotate through the players in the current round, prompting each to make a decision. After `circle_players` returns, it resets the current bet to 0.
    ///
    /// This function is used for both dealing the turn and the river in a game of Texas Hold'em poker.
    fn deal_single_card(&mut self, deck: &mut HashSet<hand::Card>, debug: bool) -> bool {
        if debug {
            println!("Dealing single cards.");
        }

        let card = deck.iter().choose(&mut rand::thread_rng()).expect("Deck ran out of cards").clone();
        self.community_cards.push(card.clone());
        deck.remove(&card);

        let has_winner: bool = self.circle_players(&mut None, &mut None, false);

        self.curr_bet = 0;

        has_winner
    }


    fn determine_winner(&mut self) {
        dbg!(&self.turn_queue);
        dbg!(&self.players_in_round);

        if self.players_in_round.len() == 1 {

            let money_earned = self.bets.values().sum();
            let player_id = self.players_in_round.iter().next().expect(PLAYER_NOT_FOUND_ERROR);
            let mut player: &mut Player = self.player_id_to_player.get_mut(player_id).expect(PLAYER_NOT_FOUND_ERROR);

            player.increment_money(money_earned);
            return;
        }




        let possible_winners = self.players_in_round.iter().cloned().collect::<HashSet<PlayerId>>();

        let mut winners: HashMap<i32, PlayerId> = HashMap::new();

        for (i, (pot, players)) in self.pots.iter().enumerate() {
            let player_to_seven_cards: HashMap<PlayerId, OnePlayerAllPossibleCards> = players.iter()
                .filter(|&p| possible_winners.contains(p))
                .fold(HashMap::<PlayerId, OnePlayerAllPossibleCards>::new(), |mut map, player_id| {
                    let player: &Player = self.player_id_to_player.get(player_id).expect(PLAYER_NOT_FOUND_ERROR);

                    let seven_cards_vec: Vec<Card> = player.get_hole_cards().iter()
                        .chain(self.community_cards.iter())
                        .cloned()
                        .collect::<Vec<_>>();
                    let seven_cards: OnePlayerAllPossibleCards = OnePlayerAllPossibleCards::new(seven_cards_vec);

                    map.insert(*player_id, seven_cards);
                    map
                });

            let winner_id: PlayerId = OnePlayerAllPossibleCards::get_winner(&player_to_seven_cards);

            // let winner = self.player_id_to_player.get(&winner_id).expect(PLAYER_NOT_FOUND_ERROR);

            winners.insert(*pot, winner_id);
            // winner.increment_money(*pot);
            //
            // if i == self.pots.len() - 1 {
            //     println!("The winner is: {}", winner.get_name());
            //     println!("With a hand of: {}", player_to_seven_cards.get(&winner_id).expect(PLAYER_NOT_FOUND_ERROR).to_string());
            //     println!("They now have: {}", winner.get_money());
            // }
        }

        for (player_id, bet) in self.bets.iter().filter(|(id, _)| { possible_winners.contains(id) }) {
            let smaller_bets = self.pots.keys().filter(|&x| x <= bet).cloned().collect::<Vec<_>>();

            let mut sofar = 0;
            let mut multiplier = 1;

            for &sub_bet in smaller_bets.iter().rev() {
                if winners.get(&sub_bet).expect("Bet value was not found in winners map") != player_id {
                    sofar = max(sofar - sub_bet, 0);
                    break;
                }

                if sub_bet > sofar {
                    sofar = sub_bet;
                    multiplier = self.pots.get(&sub_bet).expect("Bet value was not found in pots map").len()
                }
            }

            let money_earned = sofar * (multiplier as i32);

            let player = self.player_id_to_player.get_mut(player_id).expect(PLAYER_NOT_FOUND_ERROR);
            player.increment_money(money_earned);
        }
    }


    /// Prompts the current player to make a decision for their turn.
    ///
    /// # Parameters
    ///
    /// * `prev_contributions`: A mutable reference to a `HashMap` that maps `PlayerId`s to the amount they have contributed to the pot in the current round.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no players in the current round, or if the player tries to raise by an amount less than the current bet.
    ///
    /// # Notes
    ///
    /// This function prompts the current player to fold, raise, or check/call, depending on the current state of the game. It then updates the game state based on the player's decision.
    ///
    /// This function does not return any value.
    fn ask_player(&mut self, prev_contributions: &mut HashMap<PlayerId, i32>) {
        Self::print_turn_state(&self);
        let community_vec = self.format_community_cards();

        let player_id = self.turn_queue.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
        let (curr_money, curr_contribution) = {
            // Limiting the scope of the mutable borrow of self here
            let player = self.player_id_to_player.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR);
            let curr_money = player.get_money();
            let curr_contribution = prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0);

            println!();
            Self::print_cards(community_vec, player);
            (curr_money, curr_contribution)
        };

        self.print_pot_state(player_id, curr_money, curr_contribution);

        if curr_money == 0 {
            println!("Skipping your turn because you went all in.");
            self.turn_queue.push_back(player_id);
            return;
        }

        let player = self.player_id_to_player.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR);
        let prev_contribution = prev_contributions.get(&player_id).cloned().unwrap_or(0);

        // prompt the player
        match self.curr_bet.checked_sub(prev_contribution).expect("Something went wrong here") {
            0 => print!("{}, Would you like to fold, raise, go all in, or check? ", player.get_name()),
            _ => print!("{}, Would you like to fold, raise, go all in, or call? ", player.get_name())
        }
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();


        match input {
            "fold" => {
                // match self.bets.get(&player_id) {
                //     Some(prev_bet) => {
                //         self.pots
                //             .iter_mut()
                //             .for_each( |(bet, mut player_ids)|
                //                 if *bet <= *prev_bet {
                //                     // self.pots.get_mut(bet).unwrap().remove(&player_id);
                //                     player_ids.remove(&player_id);
                //                 }
                //             )
                //     }
                //     None => {}
                // }
                self.players_in_round.remove(&player_id);
            },
            "raise" => {
                print!("Raise by how much? ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().parse::<i32>().unwrap();

                if input < self.curr_bet {
                    panic!("Raise must be at least {}", self.curr_bet);
                }

                let this_bet = input - prev_contribution;

                self.make_player_bet(player_id, input, prev_contribution);

                prev_contributions.insert(player_id, prev_contribution + this_bet);
                self.bet_this_round.insert(player_id);
                self.turn_queue.push_back(player_id);

                self.has_raised = true;
            },
            "all in" => {
                self.handle_all_in(&player_id, prev_contribution);
                // prev_contributions.insert(player_id, *self.bets.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR));
                // self.prev_contributions.
                // self.bet_this_round.insert(player_id);
                // self.turn_queue.push_back(player_id);
            },
            _ => {
                match self.curr_bet {
                    0 => println!("{} has checked", player.get_name()),
                    _ => println!("{} has called", player.get_name())
                }
                // let prev_contribution = prev_contributions.get(&player_id).cloned().unwrap_or(0);
                let this_bet = self.curr_bet - prev_contribution;

                self.make_player_bet(player_id, self.curr_bet, prev_contribution);

                prev_contributions.insert(player_id, prev_contribution + this_bet);
                self.bet_this_round.insert(player_id);
                self.turn_queue.push_back(player_id);
            }
        };
    }

    fn handle_all_in(&mut self, player_id: &PlayerId, prev_contribution: i32) {
        let player: &Player = self.player_id_to_player.get(player_id).expect(PLAYER_NOT_FOUND_ERROR);
        let all_in_amount: i32 = player.get_money() + self.bets.get(player_id).unwrap_or(&0);

        // self.add_to_all_bets_up_to_highest(player_id, all_in_amount);
        self.make_player_bet(*player_id, all_in_amount, prev_contribution);

        let higher_bets: Vec<i32> = self.pots.keys().filter(|&&pot| pot > all_in_amount).cloned().collect();

        let players_with_higher_bets: HashSet<PlayerId> = higher_bets
            .iter()
            .fold(HashSet::new(), |mut acc: HashSet<PlayerId>, bet| {
                acc.extend(self.pots.get_mut(bet).expect("Expected to find bet in pot map").iter());
                acc
            });

        players_with_higher_bets
            .iter()
            .for_each(
                |player: &PlayerId| {
                    self.pots.get_mut(&all_in_amount).expect("Expected to find bet in pot map").insert(*player);
            });


        //
        // self.
        // self.bets.insert(*player_id, all_in_amount);
        // self.curr_bet = max(self.curr_bet, all_in_amount);
    }


    fn print_cards(mut community_vec: VecDeque<String>, player: &Player) {
        let mut player_vec = player.format_hole_cards();
        player_vec.push_front(utils::get_dashes_for_longest_string(player_vec.clone()));
        player_vec.push_front("Your Cards".to_string());

        community_vec.push_front("-".repeat(community_vec.iter().max_by_key(|x| { x.len() }).expect("Error here").len()));
        community_vec.push_front("Community Cards:".to_string());
        let cards_display_str = utils::format_next_to_each_other(vec!(player_vec, community_vec));
        println!("{}", cards_display_str);
        println!();
    }


    fn print_pot_state(&mut self, player_id: PlayerId, curr_money: i32, curr_contribution: i32) {
        // let pot_str = vec!["Pot".to_string(), utils::dashes(6), self.pot.to_string()];
        dbg!(&self.pots);
        // let pot_str = vec!["Pot".to_string(), utils::dashes(6), self.get_total_pot_size().to_string()];
        let bet_str = vec!["Table's Current Bet".to_string(), utils::dashes(18), self.curr_bet.to_string()];
        let money_in_pot_str = vec!["Your Contribution to the Pot".to_string(), utils::dashes(29), self.bets.get(&player_id).unwrap_or(&0).to_string()];
        let curr_bet_contribution = vec!["Your Current Contribution to the Bet".to_string(), utils::dashes(37), curr_contribution.to_string()];
        let money_str = vec!["Your Money".to_string(), utils::dashes(15), curr_money.to_string()];

        let output = utils::format_next_to_each_other(vec![bet_str, money_in_pot_str, curr_bet_contribution, money_str]);
        println!("{}", output);
        println!();
    }


    /// Rotates through the players in the current round, prompting each to make a decision.
    ///
    /// # Arguments
    ///
    /// * `prev_contributions_option` - A mutable reference to an `Option` that may contain a `HashMap` mapping `PlayerId`s to the amount they have contributed to the pot in the current round.
    /// * `prev_player` - A mutable reference to an `Option` that may contain the `PlayerId` of the previous player.
    /// * `is_dealing_hold_cards` - A `bool` indicating whether the function is being called while dealing hole cards.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `false` if there is only one player left in the round, otherwise returns `true`.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no players in the current round, or if a player tries to raise by an amount less than the current bet.
    ///
    /// # Behavior
    ///
    /// This function rotates through the players in the current round, prompting each to make a decision until a condition to break the loop is met. The conditions to break the loop are:
    /// 1. There is only one player left in the round.
    /// 2. The last player to raise has been asked and all players have been asked at least once since the last raise.
    /// 3. All players have been asked at least once and the last player to be asked has matched the current bet.
    ///
    fn circle_players(&mut self, prev_contributions_option: &mut Option<HashMap<PlayerId, i32>>,  prev_player: &mut Option<PlayerId>, is_dealing_hold_cards: bool) -> bool{

        if self.turn_queue.len() == 0 {
            return true;
        }

        self.bet_this_round.clear();

        let mut prev_contributions: HashMap<PlayerId, i32>;

        match prev_contributions_option {
            Some(prev_contributions_option) => {
                prev_contributions = prev_contributions_option.clone();
            },
            None => {
                prev_contributions = HashMap::<PlayerId, i32>::new();
            }
        }


        let mut count = 0;

        loop {
            if count == 20 {break}

            if self.players_in_round.len() == 1 {
                return false;
            }



            self.ask_player(&mut prev_contributions);

            if self.turn_queue.len() == 0 {
                return true;
            }

            let player_id = self.turn_queue.front().expect(PLAYER_NOT_FOUND_ERROR).clone();
            let player = self.player_id_to_player.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR).clone();


            match prev_player {
                Some(prev_player) => {
                    if self.has_raised && player.get_player_id() == self.last_player_to_raise && self.bet_this_round.len() >= self.turn_queue.len() {
                        break;
                    }

                    if *prev_player == self.last_player_to_raise && prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0) == self.curr_bet && self.bet_this_round.len() >= self.turn_queue.len() {
                        break;
                    }
                },
                None => {
                }
            }

            *prev_player = Some(player.get_player_id());
            count += 1;
        }

        true
    }


    /// Makes a player place a bet in the game.
    ///
    /// # Parameters
    ///
    /// * `player_id`: The ID of the player who is placing the bet.
    /// * `bet`: The amount of money the player is betting.
    ///
    /// # Panics
    ///
    /// This function will panic if `player_id` does not correspond to a player in the game.
    ///
    /// # Notes
    ///
    /// This function updates the player's money, the current bet, the last player to raise, the player's total bet, and the pot.
    /// If the bet is greater than the current bet, the player becomes the last player to raise and the current bet is updated.
    fn make_player_bet(&mut self, player_id: PlayerId, bet: i32, prev_contribution: i32) {
        let difference = bet - prev_contribution;
        let mut player = self.player_id_to_player.get_mut(&player_id).expect(PLAYER_NOT_FOUND_ERROR);
        player.set_money(player.get_money() - difference);
        if bet > self.curr_bet {
            self.curr_bet = bet;
            self.last_player_to_raise = player.get_player_id();
        }
        let player_id = player.get_player_id();
        self.bets.insert(player_id, *self.bets.get(&player_id).unwrap_or(&0) + bet);

        if bet != 0 {
            self.add_to_all_bets_up_to_highest(&player_id, bet);
        }


    }


    /// Adds the player's ID to all pots that are smaller or equal to the highest bet.
    ///
    /// # Arguments
    ///
    /// * `player_id` - The ID of the player who is placing the bet.
    /// * `highest_bet` - The highest bet that the player has made.
    ///
    /// # Behavior
    ///
    /// This function iterates over the keys of the `pots` HashMap, which represent different pot sizes. For each pot size that is smaller or equal to the `highest_bet`, it adds the `player_id` to the set of players associated with that pot size.
    ///
    /// This function does not return any value.
    fn add_to_all_bets_up_to_highest(&mut self, player_id: &PlayerId, highest_bet: i32) {
        if !self.pots.contains_key(&highest_bet) {
            self.pots.insert(highest_bet, HashSet::new());
        }

        // let mut remaining = highest_bet;
        // for (bet, mut hash_set) in self.pots.iter_mut() {
        //     if remaining == 0 { break }
        //
        //     hash_set.insert(*player_id);
        //     remaining = remaining - bet;
        // }
        //
        // if remaining != 0 {
        //     self.pots.insert()
        // }

            // .fold(highest_bet, |remaining, (bet, mut hash_set): (_, &mut HashSet<PlayerId>) | {
            //     if remaining > 0 {
            //         hash_set.insert(*player_id);
            //         remaining - bet
            //     }
            //     0
            // });



        // let bet = player_id, ())
        self.pots.iter_mut()
            .filter(|&(&key, _): &(&i32, _)| key <= highest_bet)
            .for_each(|(_, mut hash_set): (_, &mut HashSet<PlayerId>) | {
                hash_set.insert(*player_id);
            });
    }

    // fn get_total_pot_size(&self) -> i32 {
    //     let mut pot = 0;
    //     for (value, subpot) in self.pots.iter() {
    //         pot += *value * (subpot.len() as i32);
    //     }
    //     pot
    // }

    fn get_total_player_money(&self) -> i32 {
        self.players.iter().map(|p| {
            let player = self.player_id_to_player.get(p).expect(PLAYER_NOT_FOUND_ERROR);
            player.get_money()
        }).sum()
    }
}
