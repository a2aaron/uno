mod cards;
mod tests;

// extern crate rand;

// use rand::Rand;

fn main() {
	use cards::Card;
	let cards: [Card; 108] = cards::get_deck();
	for card in cards.iter() {
    	println!("{:?}", card);
	}
	println!("Done");
}