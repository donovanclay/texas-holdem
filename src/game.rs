use queue::Queue;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use std::io;
use std::io::Write;
use std::collections::HashMap;

pub mod player;
pub mod hand;

pub struct Game {
    game_id: i32,
    num_players: i32,
    players: Queue<player::Player>,
    big_blind: i32,
    dealer_location: i16,
    pot: i32,
    players_in_round: Queue<player::Player>,
    community_cards: Vec<hand::Card>,
    curr_bet: i32,
    last_player_to_raise: player::PlayerId,
    bet_this_round: HashSet<player::PlayerId>,
    has_raised: bool
}


impl Game {
    pub fn new(game_id: i32, big_blind: i32) -> Game {
        Game {
            game_id,
            num_players: 0,
            players: Queue::new(),
            big_blind,
            dealer_location: 0,
            pot: 0,
            players_in_round: Queue::new(),
            community_cards: Vec::<hand::Card>::new(),
            curr_bet: big_blind,
            last_player_to_raise: 0,
            bet_this_round: HashSet::new(),
            has_raised: false
        }
    }


    pub fn add_player(&mut self, player: player::Player) {
        self.players.queue(player.clone()).unwrap();
        self.players_in_round.queue(player.clone()).unwrap();
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


    fn make_player_bet(&mut self, player: &mut player::Player, bet: i32) {
        player.set_money(player.get_money() - bet);
        // self.curr_bet = bet;
        if bet > self.curr_bet {
            self.curr_bet = bet;
            self.last_player_to_raise = player.get_player_id();
        }

        self.pot += bet;
    }


    fn ask_player(&mut self, prev_contributions: &mut HashMap<player::PlayerId, i32>) {
        let mut player = self.players_in_round.dequeue().unwrap();
        let curr_money = player.get_money();
        let curr_contribution = prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0);

        println!();
        println!("Current size of pot: {}", self.pot);
        println!("Current bet: {}", self.curr_bet);
        println!();

        // prompt the player
        println!("CURRENT BET: {}", self.curr_bet);
        match self.curr_bet {
            0 => print!("{}, you have ${}. You have {} in the pot. Would you like to fold, raise, or check? ", player.get_name(), curr_money, curr_contribution),
            _ => print!("{}, you have ${}. You have {} in the pot. Would you like to fold, raise, or call? ", player.get_name(), curr_money, curr_contribution)
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
                } else {
                    let prev_contribution = prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0);
                    let this_bet = input - prev_contribution;

                    self.make_player_bet(&mut player, this_bet);
                    prev_contributions.insert(player.get_player_id(), prev_contribution + this_bet);
                    // self.last_player_to_raise = player.get_player_id();
                    self.bet_this_round.insert(player.get_player_id());
                    self.players_in_round.queue(player).unwrap();

                    self.has_raised = true;
                }
            },
            _ => {
                match self.curr_bet {
                    0 => println!("{} has checked", player.get_name()),
                    _ => println!("{} has called", player.get_name())
                }
                let prev_contribution = prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0);
                let this_bet = self.curr_bet - prev_contribution;

                self.make_player_bet(&mut player, this_bet);
                prev_contributions.insert(player.get_player_id(), prev_contribution + this_bet);
                self.bet_this_round.insert(player.get_player_id());
                self.players_in_round.queue(player).unwrap();
            }
        };
    }

    
    fn circle_players(&mut self, prev_contributions_option: &mut Option<HashMap<player::PlayerId, i32>>,  prev_player: &mut Option<player::PlayerId>, is_dealing_hold_cards: bool) {
        // let mut player = self.players_in_round.dequeue().unwrap();
        // self.players_in_round.queue(player).unwrap();
        self.bet_this_round.clear();

        let mut prev_contributions: HashMap<player::PlayerId, i32>;

        match prev_contributions_option {
            Some(prev_contributions_option) => {
                prev_contributions = prev_contributions_option.clone();
            },
            None => {
                prev_contributions = std::collections::HashMap::<player::PlayerId, i32>::new();
            }
        }


        loop {
            if self.players_in_round.len() == 1 {
                println!("there is a WINNDER!!");
                break;
            }
            self.ask_player(&mut prev_contributions);
            let player = self.players_in_round.peek().clone().unwrap();

            // *prev_player = Some(player.get_player_id());

            match prev_player {
                Some(prev_player) => {
                    println!("Previous player id: {}", prev_player);
                    println!("last player to raise: {}", self.last_player_to_raise);
                    // println!("prev contributions: {}", prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0));
                    println!("{}", *prev_player == self.last_player_to_raise);
                    println!("{}", prev_contributions.get(&player.get_player_id()).cloned().unwrap_or(0) == self.curr_bet);
                    println!("{}", self.bet_this_round.len() >= self.players_in_round.len());

                    let group1 = (self.has_raised, player.get_player_id() == self.last_player_to_raise, self.bet_this_round.len() >= self.players_in_round.len());

                    println!("{:?}", group1);

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


    fn deal_hole_cards(&mut self) -> HashSet<hand::Card> {

        // initialize the deck
        let mut deck = hand::Card::new_full_deck();
        println!("Size of deck: {}", deck.len());

        // initialize the previous contributions map
        let mut prev_contributions = std::collections::HashMap::<player::PlayerId, i32>::new();

        // deal the hole cards to each player
        for _ in 0..self.players.len() {
            let mut player = self.players.dequeue().unwrap();
            let mut hole_cards = Vec::<hand::Card>::new();
            for _ in 0..2 {
                let card = deck.iter().choose(&mut rand::thread_rng()).unwrap().clone();
                hole_cards.push(card);
                deck.remove(&card);
            }

            player.set_hole_cards(hole_cards.clone());
            self.players.queue(player.clone()).unwrap();
        }

        // print out the hole cards for each player
        for _ in 0..self.players.len() {
            let player = self.players.dequeue().unwrap();
            println!("Player {} has hole cards: {:?}", player.get_name(), player.get_hole_cards());
            self.players.queue(player).unwrap();
        }

        let mut num_players_in_round = self.players_in_round.len() as i32;

        println!("Size of deck: {}", deck.len());
        println!("Number of players this round: {}", num_players_in_round);
        println!("Size of blinds: {}, {}", self.big_blind, self.big_blind / 2);

        // have the small blind and big blind pay
        let mut prev_player: player::PlayerId;

        let mut player = self.players_in_round.dequeue().unwrap();
        prev_player = player.get_player_id();
        player.set_money(player.get_money() - self.big_blind / 2);
        prev_contributions.insert(player.get_player_id(), self.big_blind / 2);
        self.pot += self.big_blind / 2;
        self.last_player_to_raise = player.get_player_id();
        self.players_in_round.queue(player).unwrap();

        let mut player = self.players_in_round.dequeue().unwrap();
        prev_player = player.get_player_id();
        player.set_money(player.get_money() - self.big_blind);
        prev_contributions.insert(player.get_player_id(), self.big_blind);
        self.pot += self.big_blind;
        self.last_player_to_raise = player.get_player_id();
        self.players_in_round.queue(player).unwrap();


        self.has_raised = false;
        self.circle_players(&mut Some(prev_contributions), &mut Some(prev_player), true);

        self.curr_bet = 0;
        
        deck
    }


    fn deal_flop(&mut self, deck: &mut HashSet<hand::Card>) {
        let mut community_cards = Vec::<hand::Card>::new();

        for _ in 0..3 {
            let card = deck.iter().choose(&mut rand::thread_rng()).unwrap().clone();
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


    fn deal_turn(&mut self, deck: &mut HashSet<hand::Card>) {
        let card = deck.iter().choose(&mut rand::thread_rng()).unwrap().clone();
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


    fn deal_river(&mut self, deck: &mut HashSet<hand::Card>) {
        let card = deck.iter().choose(&mut rand::thread_rng()).unwrap().clone();
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

    }


    pub fn start_game(&mut self) {
        println!("Game {} has started!", self.game_id);

        let max_round = 5;
        let mut round = 0;

        // while self.players.len() > 1 && round < max_round {
            let mut deck = self.deal_hole_cards();
            self.deal_flop(&mut deck);
            self.deal_turn(&mut deck);
            self.deal_river(&mut deck);
            self.determine_winner();
            self.dealer_location = (self.dealer_location + 1) % self.players.len() as i16;
            // round += 1;
            // self.players.rotate();
            //  rotate the dealers and players queue
        // }
        
    }
}