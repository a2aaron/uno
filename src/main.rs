mod cards;

extern crate rand;

use rand::Rand;

fn main() {
	let rng = &mut rand::thread_rng();
	println!("{:?}", cards::Card::rand(rng));

}