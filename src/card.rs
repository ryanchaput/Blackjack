//Represents the four possible suits in a deck of cards
#[derive(Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

//Defines a card struct
//A card has a numeric value 1-13 and a suit
#[derive(Debug)]
pub struct Card {
    value: u32,
    suit: Suit,
}

impl Card {
    pub fn new(value: u32, suit: Suit) -> Card {
        Card { value, suit }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}