mod cards;

extern crate rand;

// use rand::Rand;

fn main() {
    let mut counter = cards::Card::new();
    for i in 0..1000 {
        let x = counter.next().unwrap();
        println!("{:?}", x);
    }
}
