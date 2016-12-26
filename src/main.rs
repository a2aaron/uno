mod cards;

extern crate rand;

// use rand::Rand;

fn main() {
    let counter = cards::Card::new();
    for card in counter {
        println!("{:?}", card);
    }
}