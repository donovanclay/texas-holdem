use queue::Queue;
use std::collections::HashSet;
use combinations::Combinations;
// use itertools::iproduct;
use std::iter::FromIterator;

pub struct Player {
    player_id: i32,
    name: String,
    money: i32
}

impl Player {
    pub fn new(player_id: i32, name: String, money: i32) -> Player {
        Player {
            player_id,
            name,
            money
        }
    }
    
    pub fn get_player_id(&self) -> i32 {
        self.player_id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

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


#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

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


#[derive(Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct Card {
    suit: Suit,
    value: i8
}

impl Card {
    pub fn new(suit: Suit, value: i8) -> Card {
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

pub struct Hand {
    cards: HashSet<Card>
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: HashSet::new()
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.insert(card);
    }

    pub fn get_cards(&self) -> &HashSet<Card> {
        &self.cards
    }

    pub fn check_pair(&self) -> Option<Vec::<Vec<Card>>> {
        let all_pairs = Combinations::new(self.cards.iter().cloned().collect(), 2);
        let mut pairs = Vec::<Vec<Card>>::new();
        for pair in all_pairs {
            if pair[0].value == pair[1].value {
                pairs.push(pair);
            }
        }

        match pairs.len() {
            0 => None,
            _ => Some(pairs)
        }
    }

    pub fn check_two_pair(&self) -> Option<HashSet::<Vec<Card>>> {
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
            _ => Some(two_pairs)
        }
    }


    pub fn check_three_of_a_kind(&self) -> Option<HashSet::<Vec<Card>>> {
        let all_combinations = Combinations::new(self.cards.iter().cloned().collect(), 3);
        let mut three_of_a_kinds = HashSet::<Vec<Card>>::new();
        for combination in all_combinations {
            if combination[0].value == combination[1].value && combination[1].value == combination[2].value {
                three_of_a_kinds.insert(combination);
            }
        }

        match three_of_a_kinds.len() {
            0 => None,
            _ => Some(three_of_a_kinds)
        }
    }


    pub fn check_four_of_a_kind(&self) -> Option<HashSet::<Vec<Card>>> {
        let all_combinations = Combinations::new(self.cards.iter().cloned().collect(), 4);
        let mut four_of_a_kinds = HashSet::<Vec<Card>>::new();
        for combination in all_combinations {
            if combination[0].value == combination[1].value && combination[1].value == combination[2].value && combination[2].value == combination[3].value {
                four_of_a_kinds.insert(combination);
            }
        }
        
        match four_of_a_kinds.len() {
            0 => None,
            _ => Some(four_of_a_kinds)
        }
    }


    pub fn check_full_house(&self) -> HashSet::<Vec<Card>> {
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

        full_houses
    }


    pub fn check_flush(&self) -> bool {
        let mut suits = HashSet::<Suit>::new();
        for card in &self.cards {
            suits.insert(card.suit.clone());
        }

        suits.len() == 1
    }

    pub fn check_royal(&self) -> bool {
        let mut values = Vec::<i8>::new();
        for card in &self.cards {
            values.push(card.value);
        }
        values.sort();
        values == vec![1, 10, 11, 12, 13]
    }

    pub fn check_straight(&self) -> bool {
        let mut values = Vec::<i8>::new();
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
                        println!("hello!");
                    }
                }
            }
        }
    }
}


// pub struct AllPossibleHands {
//     hands: HashSet<Hand>
// }


// impl AllPossibleHands {
//     pub fn new() -> AllPossibleHands {
//         AllPossibleHands {
//             hands: HashSet::new()
//         }
//     }

//     // pub fn add_hand(&mut self, hand: hand) {
//     //     self.hands.insert(hand);
//     // }

//     pub fn get_hands(&self) -> &HashSet<Hand> {
//         &self.hands
//     }
// }

