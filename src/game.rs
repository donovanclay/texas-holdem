use queue::Queue;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;

pub mod player;
pub mod hand;

pub struct Game {
    game_id: i32,
    num_players: i32,
    players: Queue<player::Player>,
    big_blind: i32,
    dealer_location: i16,
    pot: i32,
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
        }
    }


    pub fn add_player(&mut self, player: player::Player) {
        self.players.queue(player).unwrap();
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


    fn deal_hole_cards(&mut self) -> HashSet<hand::Card>{
        let mut deck = HashSet::<hand::Card>::new();
        for suit in vec![hand::Suit::Hearts, hand::Suit::Diamonds, hand::Suit::Clubs, hand::Suit::Spades] {
            for rank in 2..=14 {
                deck.insert(hand::Card::new(suit, rank));
            }
        }
        println!("Size of deck: {}", deck.len());
        for _ in 0..self.players.len() {
            let mut player = self.players.dequeue().unwrap();
            let mut hole_cards = Vec::<hand::Card>::new();
            for _ in 0..2 {
                let card = deck.iter().choose(&mut rand::thread_rng()).unwrap().clone();
                hole_cards.push(card);
                deck.remove(&card);
            }

            player.set_hole_cards(hole_cards.clone());

            // println!("Player {} has hole cards: {:?}", player.get_name(), hole_cards);
            self.players.queue(player.clone()).unwrap();
        }

        for _ in 0..self.players.len() {
            let player = self.players.dequeue().unwrap();
            println!("Player {} has hole cards: {:?}", player.get_name(), player.get_hole_cards());
            self.players.queue(player).unwrap();
        }

        println!("Size of deck: {}", deck.len());
        println!("Number of players: {}", self.players.len());

        

        deck
    }


    fn deal_flop(&mut self, deck: &mut HashSet<hand::Card>) {}


    fn deal_turn(&mut self, deck: &mut HashSet<hand::Card>) {}


    fn deal_river(&mut self, deck: &mut HashSet<hand::Card>) {}


    fn determine_winner(&mut self) {}


    pub fn start_game(&mut self) {
        println!("Game {} has started!", self.game_id);

        let max_round = 5;
        let mut round = 0;

        while self.players.len() > 1 && round < max_round {
            let mut deck = self.deal_hole_cards();
            self.deal_flop(&mut deck);
            self.deal_turn(&mut deck);
            self.deal_river(&mut deck);
            self.determine_winner();
            self.dealer_location = (self.dealer_location + 1) % self.players.len() as i16;
            round += 1;
            // self.players.rotate();
            //  rotate the dealers and players queue
        }
        
    }
}