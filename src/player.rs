use crate::card::Card;

#[derive(Debug)]
pub struct Player {
    cards: Vec<Card>,
    value: u32,
}

impl Player {
    pub fn new(&self) -> Player {
        Player { cards: Vec::new(), value: 0 }
    }

    //Deal the given card to the Player
    //Updates their cards Vec and value
    pub fn get_card(&mut self, card: Card) {
        self.cards.push(card);
        self.value += card.get_value();
    }
}
