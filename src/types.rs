use queue::Queue;
use std::collections::HashSet;
use combinations::Combinations;
use std::iter::zip;
use std::iter::FromIterator;

pub struct Player {
    player_id: i32,
    name: String,
    money: i32
}

/// Represents a player in a Texas Hold'em game.
impl Player {
    /// Creates a new player with the given player ID, name, and initial amount of money.
    ///
    /// # Arguments
    ///
    /// * `player_id` - The unique identifier for the player.
    /// * `name` - The name of the player.
    /// * `money` - The initial amount of money the player has.
    ///
    /// # Returns
    ///
    /// A new `Player` instance.
    pub fn new(player_id: i32, name: String, money: i32) -> Player {
        Player {
            player_id,
            name,
            money
        }
    }
    
    /// Returns the player ID.
    ///
    /// # Returns
    ///
    /// The player ID.
    pub fn get_player_id(&self) -> i32 {
        self.player_id
    }

    /// Returns the player's name.
    ///
    /// # Returns
    ///
    /// The player's name.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Returns the amount of money the player has.
    ///
    /// # Returns
    ///
    /// The amount of money the player has.
    pub fn get_money(&self) -> i32 {
        self.money
    }
}


impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            player_id: self.player_id,
            name: self.name.clone(),
            money: self.money
        }
    }
}


pub struct Game {
    game_id: i32,
    num_players: i32,
    players: Queue<Player>
}


impl Game {
    pub fn new(game_id: i32) -> Game {
        Game {
            game_id,
            num_players: 0,
            players: Queue::new()
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.queue(player).unwrap();
        self.num_players += 1;
    }

    pub fn get_num_players(&self) -> i32 {
        self.num_players
    }

    pub fn get_game_id(&self) -> i32 {
        self.game_id
    }
}


#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

/// Implements the `Display` trait for the `Suit` enum.
///
/// This allows instances of the `Suit` enum to be formatted as strings using the `write!` macro.
impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Spades => write!(f, "Spades")
        }
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub struct Card {
    suit: Suit,
    value: i32
}

impl Card {
    pub fn new(suit: Suit, value: i32) -> Card {
        Card {
            suit,
            value
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    StraightFlush,
    Royal,
    RoyalFlush
}

impl std::fmt::Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandType::HighCard => write!(f, "High Card"),
            HandType::Pair => write!(f, "Pair"),
            HandType::TwoPair => write!(f, "Two Pair"),
            HandType::ThreeOfAKind => write!(f, "Three of a Kind"),
            HandType::FourOfAKind => write!(f, "Four of a Kind"),
            HandType::FullHouse => write!(f, "Full House"),
            HandType::Flush => write!(f, "Flush"),
            HandType::Straight => write!(f, "Straight"),
            HandType::StraightFlush => write!(f, "Straight Flush"),
            HandType::Royal => write!(f, "Royal"),
            HandType::RoyalFlush => write!(f, "Royal Flush")
        }
    }
}

pub struct Hand {
    cards: HashSet<Card>
}

impl Hand {
    pub fn new(input_cards: HashSet::<Card>) -> Hand {
        if input_cards.len() != 5 {
            panic!("A hand must have exactly 5 cards");
        }

        for card in &input_cards {
            if card.value < 2 || card.value > 14 {
                panic!("Card value must be between 2 and 14");
            }
        }
        Hand {
            cards: input_cards.clone()
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.insert(card);
    }

    pub fn get_cards(&self) -> &HashSet<Card> {
        &self.cards
    }

    pub fn check_high_card(&self) -> Option<(HashSet<Vec<Card>>, i32, Vec<Card>)> {
        let cards = self.cards.iter().cloned().collect::<Vec<Card>>();
        let highest_card = cards.iter().max_by(|card1, card2| card1.value.cmp(&card2.value)).unwrap();
        let transformed_set: HashSet<Vec<Card>> = self.cards.iter().map(|card| vec![card.clone()]).collect();

        Some((transformed_set, highest_card.value, vec![highest_card.clone()]))
    }

    pub fn check_pair(&self) -> Option<(HashSet::<Vec<Card>>, i32, Vec<Card>)> {
        let all_pairs = Combinations::new(self.cards.iter().cloned().collect(), 2);
        let mut pairs = HashSet::<Vec<Card>>::new();
        for pair in all_pairs {
            if pair[0].value == pair[1].value {
                pairs.insert(pair);
            }
        }

        let highest_pair: (i32, Vec<Card>) = pairs.iter().map(|pair| (pair[0].value, pair.clone()))
        .max_by(|(value1, _pair1), (value2, _pair2)| value1.cmp(value2))?;

        match pairs.len() {
            0 => None,
            _ => Some((pairs, highest_pair.0, highest_pair.1))
        }
    }

    pub fn check_two_pair(&self) -> Option<(HashSet::<Vec<Card>>, i32, Vec<Card>)> {
        let all_combinations = Combinations::new(self.cards.iter().cloned().collect(), 2);
        let all_combinations_vec = all_combinations.collect::<Vec<Vec<Card>>>();
        let mut two_pairs = HashSet::<Vec<Card>>::new();
        for combination in all_combinations_vec.iter() {
            if combination[0].value == combination[1].value {
                for combination2 in all_combinations_vec.clone() {
                    if combination2[0].value == combination2[1].value && combination2[0].value != combination[0].value {
                        let mut two_pair = Vec::<Card>::new();
                        two_pair.extend(combination.clone());
                        two_pair.extend(combination2);
                        two_pairs.insert(two_pair);
                    }
                }
            }
        }

        match two_pairs.len() {
            0 => None,
            _ => {
                let highest_two_pair = two_pairs.iter().max_by(|pair1, pair2| {
                    let pair1_values = pair1.iter().map(|card| card.value).collect::<Vec<i32>>();
                    let pair2_values = pair2.iter().map(|card| card.value).collect::<Vec<i32>>();
                    pair1_values.iter().max().unwrap().cmp(pair2_values.iter().max().unwrap())
                }).unwrap();

                let highest_value: i32 = highest_two_pair.iter().map(|card| card.value).sum();


                Some((two_pairs.clone(), highest_value, highest_two_pair.clone()))
            }
        }
    }


    pub fn check_three_of_a_kind(&self) -> Option<(HashSet::<Vec<Card>>, i32, Vec<Card>)> {
        let all_combinations = Combinations::new(self.cards.iter().cloned().collect(), 3);
        let mut three_of_a_kinds = HashSet::<Vec<Card>>::new();
        for combination in all_combinations {
            if combination[0].value == combination[1].value && combination[1].value == combination[2].value {
                three_of_a_kinds.insert(combination);
            }
        }

        match three_of_a_kinds.len() {
            0 => None,
            _ => {
                let highest_three_of_a_kind = three_of_a_kinds.iter().max_by(|pair1, pair2| {
                    let pair1_values = pair1.iter().map(|card| card.value).collect::<Vec<i32>>();
                    let pair2_values = pair2.iter().map(|card| card.value).collect::<Vec<i32>>();
                    pair1_values.iter().max().unwrap().cmp(pair2_values.iter().max().unwrap())
                }).unwrap();

                let highest_value: i32 = highest_three_of_a_kind.iter().map(|card| card.value).sum();

                Some((three_of_a_kinds.clone(), highest_value, highest_three_of_a_kind.clone()))
            }
        }
    }


    pub fn check_four_of_a_kind(&self) -> Option<(HashSet::<Vec<Card>>, i32, Vec<Card>)> {
        let all_combinations = Combinations::new(self.cards.iter().cloned().collect(), 4);
        let mut four_of_a_kinds = HashSet::<Vec<Card>>::new();
        for combination in all_combinations {
            if combination[0].value == combination[1].value && combination[1].value == combination[2].value && combination[2].value == combination[3].value {
                four_of_a_kinds.insert(combination);
            }
        }
        
        match four_of_a_kinds.len() {
            0 => None,
            _ => {
                let highest_four_of_a_kind = four_of_a_kinds.iter().max_by(|pair1, pair2| {
                    let pair1_values = pair1.iter().map(|card| card.value).collect::<Vec<i32>>();
                    let pair2_values = pair2.iter().map(|card| card.value).collect::<Vec<i32>>();
                    pair1_values.iter().max().unwrap().cmp(pair2_values.iter().max().unwrap())
                }).unwrap();

                let highest_value = highest_four_of_a_kind.iter().map(|card| card.value).sum();

                Some((four_of_a_kinds.clone(), highest_value, highest_four_of_a_kind.clone()))
            }
        }
    }


    pub fn check_full_house(&self) -> Option<(HashSet::<Vec<Card>>, i32, Vec<Card>)> {
        let all_triplets = Combinations::new(self.cards.iter().cloned().collect(), 3);
        let mut full_houses = HashSet::<Vec<Card>>::new();
        
        for triplet in all_triplets {
            let pair = (&self.cards.clone() - &HashSet::<Card>::from_iter(triplet.clone())).iter().cloned().collect::<Vec<Card>>();
            if triplet[0].value == triplet[1].value && triplet[1].value == triplet[2].value {
                if pair[0].value == pair[1].value {
                    let mut full_house = Vec::<Card>::new();
                    full_house.extend(triplet);
                    full_house.extend(pair);
                    full_houses.insert(full_house);
                }
            }
        }

        match full_houses.len() {
            0 => None,
            _ => {
                let highest_full_house = full_houses.iter().max_by(|pair1, pair2| {
                    let pair1_values = pair1.iter().map(|card| card.value).collect::<Vec<i32>>();
                    let pair2_values = pair2.iter().map(|card| card.value).collect::<Vec<i32>>();
                    pair1_values.iter().max().unwrap().cmp(pair2_values.iter().max().unwrap())
                }).unwrap();

                let highest_value = highest_full_house.iter().map(|card| card.value).sum();

                Some((full_houses.clone(), highest_value, highest_full_house.clone()))
            }
        }
    }


    pub fn check_flush(&self) -> bool {
        let mut suits = HashSet::<Suit>::new();
        for card in &self.cards {
            suits.insert(card.suit.clone());
        }

        suits.len() == 1
    }

    pub fn check_royal(&self) -> bool {
        let mut values = Vec::<i32>::new();
        for card in &self.cards {
            values.push(card.value);
        }
        values.sort();
        values == vec![1, 10, 11, 12, 13]
    }

    pub fn check_straight(&self) -> bool {
        let mut values = Vec::<i32>::new();
        for card in &self.cards {
            values.push(card.value);
        }
        values.sort();
        // check for ace, 10, jack, queen, king
        if values == vec![1, 10, 11, 12, 13] {
            return true;
        }
        for i in 0..values.len() - 1 {
            if values[i + 1] - values[i] != 1 {
                return false;
            }
        }

        true
    }


    pub fn check_straight_flush(&self) -> bool {
        self.check_straight() && self.check_flush()
    }


    pub fn check_royal_flush(&self) -> bool {
        self.check_royal() && self.check_flush()
    }


    pub fn check_hand(&self) {
        let is_flush = self.check_flush();
        let is_royal = self.check_royal();
        let is_straight = self.check_straight();

        let high_card_output = self.check_high_card();
        let pairs_output= self.check_pair();
        let two_pairs_output = self.check_two_pair();
        let triplets_output = self.check_three_of_a_kind();
        let four_of_a_kind_output = self.check_four_of_a_kind();
        let full_houses_output = self.check_full_house();

        let outputs = vec![four_of_a_kind_output, full_houses_output, triplets_output, two_pairs_output, pairs_output, high_card_output];
        let hand_types_ranked = vec![HandType::FourOfAKind, HandType::FullHouse, HandType::ThreeOfAKind, HandType::TwoPair, HandType::Pair, HandType::HighCard];

        match is_flush {
            true => {
                match is_royal {
                    true => {
                        println!("Royal Flush");
                    },
                    false => {
                        match is_straight {
                            true => {
                                println!("Straight Flush");
                            },
                            false => {
                                println!("Flush");
                            }
                        }
                    }
                }
            },
            false => {
                match is_straight {
                    true => println!("Straight"),
                    false => {
                        for (output, hand_type) in zip(outputs, hand_types_ranked) {
                            match output {
                                Some((_, highest_value, highest_hand)) => {
                                    println!("{}", hand_type);
                                    break;
                                },
                                None => {
                                    // println!("No hand");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

