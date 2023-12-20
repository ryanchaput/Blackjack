use crate::card::Card;

mod card;
mod deck;
mod player;

fn main() {
    println!("Welcome to Blackjack!");

    let pcard1: Card; //Placeholders
    let pcard2: Card;

    let psum = pcard1.get_value() + pcard2.get_value();

    let dcard1: Card;
    let dcard2: Card;

    let dsum: u32 = dcard1.get_value() + dcard2.get_value();

    //println!("Your cards are: {pcard1}, {pcard2}, for a total value of: {psum}");

    //println!("The dealer's cards are: {dcard1}, {dcard2}")

    println!("Hit or stand?");
}
