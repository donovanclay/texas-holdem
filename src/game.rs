use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::collections::HashMap;
use std::io;
use std::io::Write;

use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;

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
/// * `pot`: An `i32` that represents the current pot amount.
/// * `community_cards`: A `Vec` of `hand::Card`s that represents the community cards.
/// * `curr_bet`: An `i32` that represents the current bet amount.
/// * `last_player_to_raise`: A `PlayerId` that represents the last player to raise.
/// * `bet_this_round`: A `HashSet` of `PlayerId`s that represents the players who have bet in this round.
/// * `has_raised`: A `bool` that indicates whether a player has raised in the current round.
/// * `bets`: A `HashMap` that maps `PlayerId`s to the amount they have bet.
pub struct Game {
    game_id: i32,
    num_players: i32,
    players: VecDeque<PlayerId>,
    players_in_round: VecDeque<PlayerId>,
    player_id_to_player: HashMap<PlayerId, Player>,
    big_blind: i32,
    dealer_location: i16,
    pot: i32,
    community_cards: Vec<hand::Card>,
    curr_bet: i32,
    last_player_to_raise: PlayerId,
    bet_this_round: HashSet<PlayerId>,
    has_raised: bool,
    bets: HashMap<PlayerId, i32>
}


impl Game {
    pub fn new(game_id: i32, big_blind: i32) -> Game {
        Game {
            game_id,
            num_players: 0,
            players: VecDeque::new(),
            players_in_round: VecDeque::new(),
            player_id_to_player: HashMap::new(),
            big_blind,
            dealer_location: 0,
            pot: 0,
            community_cards: Vec::<hand::Card>::new(),
            curr_bet: big_blind,
            last_player_to_raise: 0,
            bet_this_round: HashSet::new(),
            has_raised: false,
            bets: HashMap::new()
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push_back(player.get_player_id());
        self.players_in_round.push_back(player.get_player_id());
        self.player_id_to_player.insert(player.get_player_id(), player.clone());
        self.num_players += 1;
    }


    pub fn get_num_players(&self) -> i32 {
        self.num_players
    }


    pub fn get_game_id(&self) -> i32 {
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
        let players = self.players_in_round.clone();
        let width =  players.front().expect(PLAYER_NOT_FOUND_ERROR).to_string().len() / 2;

        let players_str = players.iter()
            .map(|player| player.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let turn_state = " Turn State ";
        let dashes = "-".repeat(max(players_str.len() - turn_state.len(), 0) / 2);

        println!();
        println!("{}", dashes.clone() + turn_state + &dashes);
        println!("{}", " ".repeat(width) + "|");
        println!("{}", " ".repeat(width) + "|");
        println!("{}", " ".repeat(width) + "▼");
        println!("{}", players_str);
        println!();
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
        let mut round = 0;

        // while self.players.len() > 1 && round < max_round {
        self.bets.clear();
        let mut deck = self.deal_hole_cards(debug);
        self.deal_flop(&mut deck);

        // Deal the turn.
        self.deal_single_card(&mut deck);
        // Deal the river.
        self.deal_single_card(&mut deck);
        self.determine_winner();
        self.dealer_location = (self.dealer_location + 1) % self.players.len() as i16;
        // round += 1;
        // self.players.rotate();
        //  rotate the dealers and players queue
        // }
    }

    /// Deals hole cards to each player and handles the small and big blinds.
    ///
    /// # Parameters
    ///
    /// * `debug`: A `bool` indicating whether to print debug information.
    ///
    /// # Returns
    ///
    /// A `HashSet` of `hand::Card`s representing the remaining deck after dealing the hole cards.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no players in the current round, or if a player tries to raise by an amount less than the current bet.
    ///
    /// # Notes
    ///
    /// This function initializes a full deck of cards, deals two hole cards to each player, and handles the small and big blinds. It then calls `circle_players` to rotate through the players in the current round, prompting each to make a decision. After `circle_players` returns, it resets the current bet to 0.
    ///
    /// If `debug` is `true`, this function also prints debug information, such as the size of the deck, the number of players in the current round, the size of the blinds, and the hole cards of each player.
    fn deal_hole_cards(&mut self, debug: bool) -> HashSet<hand::Card> {

        // initialize the deck
        let mut deck = hand::Card::new_full_deck();
        if debug {
            println!("Size of deck: {}", deck.len());
        }


        // initialize the previous contributions map
        let mut prev_contributions = HashMap::<PlayerId, i32>::new();

        // deal the hole cards to each player
        for _ in 0..self.players.len() {
            let mut player_id = self.players.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
            let mut player = self.player_id_to_player.get_mut(&player_id).expect(PLAYER_NOT_FOUND_ERROR);
            let mut hole_cards = Vec::<hand::Card>::new();
            for _ in 0..2 {
                let card = deck.iter().choose(&mut rand::thread_rng()).expect("Deck ran out of cards").clone();
                hole_cards.push(card);
                deck.remove(&card);
            }

            player.set_hole_cards(hole_cards.clone());
            self.players.push_back(player.get_player_id());
        }

        // print out the hole cards for each player
        for _ in 0..self.players.len() {
            let player_id = self.players.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
            let player = self.player_id_to_player.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR).clone();

            if debug {
                println!("Player {} has hole cards: {:?}", player.get_name(), player.get_hole_cards());
            }

            self.players.push_back(player.get_player_id());
        }

        let num_players_in_round = self.players_in_round.len() as i32;

        if debug {
            println!("Size of deck: {}", deck.len());
            println!("Number of players this round: {}", num_players_in_round);
            println!("Size of blinds: {}, {}", self.big_blind, self.big_blind / 2);
        }


        // have the small blind and big blind pay
        let mut prev_player: PlayerId;

        let player_id = self.players_in_round.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
        prev_player = player_id;

        prev_contributions.insert(player_id, self.big_blind / 2);
        self.make_player_bet(player_id, self.big_blind / 2);

        self.last_player_to_raise = player_id;
        self.players_in_round.push_back(player_id);

        let player_id = self.players_in_round.pop_front().expect(PLAYER_NOT_FOUND_ERROR);

        prev_player = player_id;

        prev_contributions.insert(player_id, self.big_blind);

        self.make_player_bet(player_id, self.big_blind);

        self.last_player_to_raise = player_id;
        self.players_in_round.push_back(player_id);


        self.has_raised = false;
        self.circle_players(&mut Some(prev_contributions), &mut Some(prev_player), true);

        self.curr_bet = 0;

        deck
    }


    /// Deals the flop (the first three community cards) in the game.
    ///
    /// # Parameters
    ///
    /// * `deck`: A mutable reference to a `HashSet` of `hand::Card`s representing the current deck.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no cards left in the deck.
    ///
    /// # Notes
    ///
    /// This function randomly selects three cards from the deck, removes them from the deck, and adds them to the community cards. It then prints the community cards and the size of the deck. After that, it calls `circle_players` to rotate through the players in the current round, prompting each to make a decision. After `circle_players` returns, it resets the current bet to 0.
    ///
    /// This function does not return any value.
    fn deal_flop(&mut self, deck: &mut HashSet<hand::Card>) {
        let mut community_cards = Vec::<hand::Card>::new();

        for _ in 0..3 {
            let card = deck.iter().choose(&mut rand::thread_rng()).expect("Deck ran out of cards").clone();
            community_cards.push(card);
            deck.remove(&card);
        }

        self.community_cards = community_cards.clone();

        println!("Community cards: ");
        for card in &self.community_cards {
            println!("{:?}", card);
        }
        println!("Size of deck: {}", deck.len());

        self.circle_players(&mut None, &mut None, false);

        self.curr_bet = 0;
    }


    /// Deals a single card to the community cards in the game.
    ///
    /// # Parameters
    ///
    /// * `deck`: A mutable reference to a `HashSet` of `hand::Card`s representing the current deck.
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
    ///
    /// This function does not return any value.
    fn deal_single_card(&mut self, deck: &mut HashSet<hand::Card>) {
        let card = deck.iter().choose(&mut rand::thread_rng()).expect("Deck ran out of cards").clone();
        self.community_cards.push(card.clone());
        deck.remove(&card);

        println!("Community cards: ");
        for card in &self.community_cards {
            println!("{:?}", card);
        }
        println!("Size of deck: {}", deck.len());

        self.circle_players(&mut None, &mut None, false);

        self.curr_bet = 0;
    }

    fn determine_winner(&mut self) {
        // let mut best_hand = hand::Hand::new(HashSet::new());
        let mut best_player_id: PlayerId = self.players_in_round.front().expect(PLAYER_NOT_FOUND_ERROR).clone();
        let mut best_score: Option<hand::HandScore> = None;

        for _ in 0..self.players_in_round.len() {
            let player_id = self.players_in_round.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
            let player = self.player_id_to_player.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR).clone();
            let mut all_cards = Vec::<hand::Card>::new();

            for card in player.get_hole_cards() {
                all_cards.push(card.clone());
            }

            for card in &self.community_cards {
                all_cards.push(card.clone());
            }

            let all_cards = hand::OnePlayerAllPossibleCards::new(all_cards);
            let hand_score = all_cards.get_highest_hand_score();

            match best_score {
                Some(ref mut best_score) => {
                    if hand_score > *best_score {
                        // best_hand = hand;
                        best_player_id = player_id.clone();
                        *best_score = hand_score;
                    }
                },
                None => {
                    // best_hand = hand;
                    best_player_id = player_id.clone();
                    best_score = Some(hand_score);
                }
            }

            self.players.push_back(player_id);
        }

        let best_player = self.player_id_to_player.get_mut(&best_player_id).expect(PLAYER_NOT_FOUND_ERROR);
        best_player.set_money(best_player.get_money() + self.pot);

        println!("The winner is: {}", best_player.get_name());
        println!("With a hand of: {}", best_score.expect("Error calculating the best score."));
        println!("They now have: {}", best_player.get_money());
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
        let community_vec = self.format_community_cards();

        let player_id = self.players_in_round.pop_front().expect(PLAYER_NOT_FOUND_ERROR);
        let (curr_money, curr_contribution) = {
            // Limiting the scope of the mutable borrow of self here
            let player = self.player_id_to_player.get_mut(&player_id).expect(PLAYER_NOT_FOUND_ERROR);
            let curr_money = player.get_money();
            let curr_contribution = prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0);

            println!();

            Self::print_cards(community_vec, player);
            (curr_money, curr_contribution)
        };

        self.print_pot_state(player_id, curr_money, curr_contribution);

        let player = self.player_id_to_player.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR);
        let prev_contribution = prev_contributions.get(&player_id).cloned().unwrap_or(0);

        // prompt the player
        match self.curr_bet.checked_sub(prev_contribution).expect("Something went wrong here") {
            0 => print!("{}, Would you like to fold, raise, or check? ", player.get_name()),
            _ => print!("{}, Would you like to fold, raise, or call? ", player.get_name())
        }
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();


        match input {
            "fold" => println!("{} has folded", player.get_name()),
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

                self.make_player_bet(player_id, this_bet);

                prev_contributions.insert(player_id, prev_contribution + this_bet);
                self.bet_this_round.insert(player_id);
                self.players_in_round.push_back(player_id);

                self.has_raised = true;
            },
            _ => {
                match self.curr_bet {
                    0 => println!("{} has checked", player.get_name()),
                    _ => println!("{} has called", player.get_name())
                }
                // let prev_contribution = prev_contributions.get(&player_id).cloned().unwrap_or(0);
                let this_bet = self.curr_bet - prev_contribution;

                self.make_player_bet(player_id, this_bet);

                prev_contributions.insert(player_id, prev_contribution + this_bet);
                self.bet_this_round.insert(player_id);
                self.players_in_round.push_back(player_id);
            }
        };
    }


    fn print_cards(mut community_vec: VecDeque<String>, player: &mut Player) {
        let mut player_vec = player.format_hole_cards();
        player_vec.push_front(utils::get_dashes_for_longest_string(player_vec.clone()));
        player_vec.push_front("Your Cards".to_string());

        community_vec.push_front("-".repeat(community_vec.iter().max_by_key(|x| { x.len() }).expect("Error here").len()));
        community_vec.push_front("Community Cards:".to_string());
        let cards_display_str = utils::print_next_to_each_other(vec!(player_vec, community_vec));
        println!("{}", cards_display_str);
        println!();
    }


    fn print_pot_state(&mut self, player_id: PlayerId, curr_money: i32, curr_contribution: i32) {
        let pot_str = vec!["Pot".to_string(), utils::dashes(6), self.pot.to_string()];
        let bet_str = vec!["Table's Current Bet".to_string(), utils::dashes(18), self.curr_bet.to_string()];
        let money_in_pot_str = vec!["Your Contribution to the Pot".to_string(), utils::dashes(29), self.bets.get(&player_id).unwrap_or(&0).to_string()];
        let curr_bet_contribution = vec!["Your Current Contribution to the Bet".to_string(), utils::dashes(37), curr_contribution.to_string()];
        let money_str = vec!["Your Money".to_string(), utils::dashes(15), curr_money.to_string()];

        let output = utils::print_next_to_each_other(vec![pot_str, bet_str, money_in_pot_str, curr_bet_contribution, money_str]);
        println!("{}", output);
        println!();
    }


    /// Rotates through the players in the current round, prompting each to make a decision.
    ///
    /// # Parameters
    ///
    /// * `prev_contributions_option`: A mutable reference to an `Option` that may contain a `HashMap` mapping `PlayerId`s to the amount they have contributed to the pot in the current round.
    /// * `prev_player`: A mutable reference to an `Option` that may contain the `PlayerId` of the previous player.
    /// * `is_dealing_hold_cards`: A `bool` indicating whether the function is being called while dealing hole cards.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no players in the current round, or if a player tries to raise by an amount less than the current bet.
    ///
    /// # Notes
    ///
    /// This function rotates through the players in the current round, prompting each to make a decision until a condition to break the loop is met. The conditions to break the loop are:
    /// 1. There is only one player left in the round.
    /// 2. The last player to raise has been asked and all players have been asked at least once since the last raise.
    /// 3. All players have been asked at least once and the last player to be asked has matched the current bet.
    ///
    /// This function does not return any value.
    fn circle_players(&mut self, prev_contributions_option: &mut Option<HashMap<PlayerId, i32>>,  prev_player: &mut Option<PlayerId>, is_dealing_hold_cards: bool) {
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


        loop {
            if self.players_in_round.len() == 1 {
                println!("there is a WINNDER!!");
                break;
            }
            self.ask_player(&mut prev_contributions);
            let player_id = self.players_in_round.front().expect(PLAYER_NOT_FOUND_ERROR).clone();
            let player = self.player_id_to_player.get(&player_id).expect(PLAYER_NOT_FOUND_ERROR).clone();


            match prev_player {
                Some(prev_player) => {
                    if self.has_raised && player.get_player_id() == self.last_player_to_raise && self.bet_this_round.len() >= self.players_in_round.len() {
                        break;
                    }

                    if *prev_player == self.last_player_to_raise && prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0) == self.curr_bet && self.bet_this_round.len() >= self.players_in_round.len() {
                        break;
                    }
                },
                None => {
                    println!("hello");
                }
            }

            *prev_player = Some(player.get_player_id());
        }

        println!();
        println!("Current size of pot: {}", self.pot);
        println!("Current bet: {}", self.curr_bet);
        println!();
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
    fn make_player_bet(&mut self, player_id: PlayerId, bet: i32) {
        let mut player = self.player_id_to_player.get_mut(&player_id).expect(PLAYER_NOT_FOUND_ERROR);
        player.set_money(player.get_money() - bet);
        if bet > self.curr_bet {
            self.curr_bet = bet;
            self.last_player_to_raise = player.get_player_id();
        }
        let player_id = player.get_player_id();
        self.bets.insert(player_id, *self.bets.get(&player_id).unwrap_or(&0) + bet);
        self.pot += bet;
    }



}