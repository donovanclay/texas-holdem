// pub mod crate::..::hand;
use crate::game::hand;

// pub mod player {
pub struct Player {
    player_id: i32,
    name: String,
    money: i32,
    hole_cards: Vec<hand::Card>
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
            money,
            hole_cards: Vec::<hand::Card>::new()
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


    pub fn get_hole_cards(&self) -> Vec<hand::Card> {
        self.hole_cards.clone()
    }


    pub fn set_hole_cards(&mut self, cards: Vec<hand::Card>) {
        self.hole_cards = cards;
    }
}


impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            player_id: self.player_id,
            name: self.name.clone(),
            money: self.money,
            hole_cards: self.hole_cards.clone()
        }
    }
}
// }