use std::collections::HashMap;
use std::collections::HashSet;
use combinations::Combinations;
use std::iter::zip;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Copy)]
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


/**
  * Represents a playing card.
  * Ace is 14, King is 13, Queen is 12, Jack is 11, and 2-10 are their respective values.
  */
#[derive(Hash, Eq, PartialEq, Clone, Ord, PartialOrd, Debug, Copy)]
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

  pub fn new_full_deck() -> HashSet<Card> {
    let mut deck = HashSet::<Card>::new();
    for suit in vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
      for value in 2..=14 {
        deck.insert(Card::new(suit, value));
      }
    }

    deck
  }
}

impl std::fmt::Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if (self.value >= 2 && self.value <= 10) {
      write!(f, "{} of {}", self.value, self.suit)
  } else {
      let value_str = match self.value {
        11 => "Jack",
        12 => "Queen",
        13 => "King",
        14 => "Ace",
        _ => ""
      };
      write!(f, "{} of {}", value_str, self.suit)
    }
  }
}

/**
  * Represents a hand type in poker.
  */
#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy)]
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


/**
  * Implements the `Ord` trait for the `HandType` enum.
  *
  * This allows instances of the `HandType` enum to be compared using the `<`, `>`, `<=`, and `>=` operators.
  */
impl Ord for HandType {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let hand_types = vec![HandType::HighCard, HandType::Pair, HandType::TwoPair, HandType::ThreeOfAKind, HandType::Straight, HandType::Flush, HandType::FullHouse, HandType::FourOfAKind, HandType::StraightFlush, HandType::RoyalFlush];
    let self_index = hand_types.iter().position(|hand_type| hand_type == self).unwrap();
    let other_index = hand_types.iter().position(|hand_type| hand_type == other).unwrap();
    self_index.cmp(&other_index)
  }
}


/**
  * Implements the `PartialOrd` trait for the `HandType` enum.
  *
  * This allows instances of the `HandType` enum to be compared using the `cmp` method.
  */
impl PartialOrd for HandType {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}


/**
  * Represents a score for a hand in poker.
  *
  * This works similar to a decimal system. Handtype takes priority over score.
  * The score is used to compare hands of the same type.
  */
#[derive(Eq, Hash, PartialEq)]
pub struct HandScore {
  hand_type: HandType,
  score: i32
}


impl std::fmt::Display for HandScore {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} with a score of {}", self.hand_type, self.score)
  }
}


/**
  * Implements the `Ord` trait for the `HandScore` struct.
  *
  * This allows instances of the `HandScore` struct to be compared using the `<`, `>`, `<=`, and `>=` operators.
  */
impl Ord for HandScore {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let c = self.hand_type.cmp(&other.hand_type);
    if c == std::cmp::Ordering::Equal {
      self.score.cmp(&other.score)
    } else {
      c
    }
  }
}


impl PartialOrd for HandScore {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl HandScore {
  pub fn get_hand_type(&self) -> HandType {
    self.hand_type
  }

  pub fn get_score(&self) -> i32 {
    self.score
  }
}


/**
  * Represents a hand of cards in poker.
  */
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

  /**
    Returns the cards in the hand.
    */
  pub fn get_cards(&self) -> &HashSet<Card> {
    &self.cards
  }

  /** Checks 
   * hello
   */
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
    values == vec![10, 11, 12, 13, 14]
  }


  pub fn check_straight(&self) -> bool {
    let mut values = Vec::<i32>::new();
    for card in &self.cards {
      values.push(card.value);
    }
    values.sort();
    // check for ace, 1, 2, 3, 4
    if values == vec![2, 3, 4, 5, 14] {
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


  fn calculate_score_for_boolean(&self) -> i32 {
    let mut score = 0;
    let values = self.cards.iter().map(|card| card.value);
    let mut values = values.collect::<Vec<i32>>();
    values.sort();
    for (i, value) in values.iter().enumerate() {
      score += value * (i + 1) as i32 * 14  as i32;
    }
    score
  }


  fn calculate_score_for_straight(&self) -> i32 {
    let mut score = 0;
    let values = self.cards.iter().map(|card| card.value);
    let mut values = values.collect::<Vec<i32>>();
    values.sort();
    if values == vec![2, 3, 4, 5, 14] {
      values = vec![1, 2, 3, 4, 5];
    }
    for (i, value) in values.iter().enumerate() {
      score += value * (i + 1) as i32 * 14  as i32;
    }
    score
  }


  fn calculate_score_for_full_house(&self) -> i32 {
    let mut score = 0;
    let values_map = self.cards.iter().map(|card| card.value).fold(HashMap::<i32, i32>::new(), |mut acc, value| {
      *acc.entry(value).or_insert(0) += 1;
      acc
    });

    for (key, value) in values_map {
      if value == 3 {
        score += key * 14;
      } else {
        score += key;
      }
    }

    score
  }


  pub fn check_hand(&self) -> HandScore {
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
    let hand_type_multiples = HashMap::<HandType, i32>::from_iter(vec![
      (HandType::HighCard, 1),
      (HandType::Pair, 6188),
      (HandType::TwoPair, 3848),
      (HandType::ThreeOfAKind, 624),
      (HandType::Straight, 1976),
      (HandType::Flush, 9880),
      (HandType::FullHouse, 9828),
      (HandType::FourOfAKind, 10764),
      (HandType::StraightFlush, 676),
      (HandType::RoyalFlush, 9100)
    ]);

    let mut output_hand_type: HandType= HandType::HighCard;
    let mut score: i32 = 0;

    match is_flush {
      true => {
        match is_royal {
          true => {
            score = self.calculate_score_for_boolean();
            output_hand_type = HandType::RoyalFlush;
          },
          false => {
            match is_straight {
              true => {
                score = self.calculate_score_for_straight();
                output_hand_type = HandType::StraightFlush;
              },
              false => {
                score = self.calculate_score_for_boolean();
                output_hand_type = HandType::Flush;
              }
            }
          }
        }
      },
      false => {
        match is_straight {
          true => {
              score = self.calculate_score_for_boolean();
              output_hand_type = HandType::Straight;
          }
          false => {
            for (output, hand_type) in zip(outputs, hand_types_ranked) {
              match output {
                Some((_, highest_value, highest_hand)) => {
                  let remaining_cards = &self.cards - &HashSet::<Card>::from_iter(highest_hand.clone());
                  let remaining_values = remaining_cards.iter().map(|card| card.value);
                  let mut remaining_values = remaining_values.collect::<Vec<i32>>();
                  remaining_values.sort();
                  
                  if hand_type == HandType::FullHouse {
                      score = self.calculate_score_for_full_house();
                      output_hand_type = hand_type;
                      break;
                  }

                  let mut sum_of_remaining = 0;

                  for (i, value) in remaining_values.iter().enumerate() {
                      sum_of_remaining += value * (i + 1) as i32 * 14  as i32;
                  }

                  output_hand_type = hand_type.clone();
                  score = sum_of_remaining + highest_value * hand_type_multiples[&hand_type];
                  break;
                },
                None => {
                  continue;
                }
              }
            }
          }
        }
      }
    }

    HandScore {
        hand_type: output_hand_type,
        score
    }
  }
}

pub struct OnePlayerAllPossibleCards {
    cards: Vec<Card>
}

impl OnePlayerAllPossibleCards {

    pub fn new(all_cards: Vec<Card>) -> OnePlayerAllPossibleCards {
        let all_cards_set = HashSet::<Card>::from_iter(all_cards.clone());
        if all_cards_set.len() != 7 {
            panic!("There must be exactly 7 cards");
        }

        OnePlayerAllPossibleCards {
            cards: all_cards.clone()
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn get_highest_hand_score(&self) -> HandScore {
        let all_combinations = Combinations::new(self.cards.clone(), 5);
        let mut highest_hand_score = HandScore {
            hand_type: HandType::HighCard,
            score: 0
        };
        for combination in all_combinations {
            let hand = Hand::new(HashSet::<Card>::from_iter(combination.clone()));
            let hand_score = hand.check_hand();
            if hand_score > highest_hand_score {
                highest_hand_score = hand_score;
            }
        }

        highest_hand_score
    }
}




