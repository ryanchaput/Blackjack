use crate::card;
use card::Suit;
use card::Card;

//Represents a deck of cards
//Each deck has 52 cards, 13 values for the 4 suits
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

//Methods involving the Deck struct
impl Deck {
    pub fn new(&self) -> Deck {
        let mut cards: Vec<Card> = Vec::new();

        let suits = vec!(Suit::Hearts,
                         Suit::Diamonds,
                         Suit::Spades,
                         Suit::Clubs);

        for s in suits {
            for u in 1..14 {
                let c = Card::new(u, s);
                cards.push(c);
            }
        }

        Deck { cards }
    }
}