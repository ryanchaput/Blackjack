mod card;
mod deck;
mod player;

fn main() {
    println!("Welcome to Blackjack!");

    let pcard1: u32 = 0; //Placeholders
    let pcard2: u32 = 0;

    let psum = pcard1 + pcard2;

    let dcard1: u32 = 0;
    let dcard2: u32 = 0;

    println!("Your cards are: {pcard1}, {pcard2}, for a total value of: {psum}");

    println!("The dealer's cards are: {dcard1}, {dcard2}")
}
