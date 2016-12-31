mod cards;
mod tests;

// extern crate rand;

// use rand::Rand;

use cards::*;
use std::io;

fn main() {
	use cards::Card;
	let mut game_state: GameState = GameState::new(4);
	// Main game loop
	loop {
		println!("Your turn player {}!", game_state.players.current_player);
		println!("Top card is {:?}", game_state.play_deck.last());
		println!("Your hand");
		for card in game_state.players.get_current_player() {
			println!("{:?}", card);
		}
		// Get card to play
		let (mut card, index) = read_card_from_stdin(&mut game_state.players);
		// Play the card
		match game_state.play_card(&mut card) {
			Err(card) => println!("Cannot play {:?}", card),
			Ok(game_state) => {
				game_state.next_player();
				game_state.players.get_current_player().remove(index);
			},	
		}
	}
}

fn read_color_from_stdin() -> cards::Color {
	loop {
		println!("What color (R/G/Y/B)?");
		let mut input = String::new();
		let result = io::stdin().read_line(&mut input);
		input.pop(); // Remove trailing newline
		use cards::Color::*;
		match input.as_ref() {
			"R" | "r" => return Red,
			"G" | "g" => return Green,
			"B" | "b" => return Blue,
			"Y" | "y" => return Yellow,
			_ => {},
		}
	}
}

fn read_card_from_stdin<'a>(players:&'a mut Players) -> (Card, usize) {
	let mut card_index: usize;
	loop {
		// Minus 1 because humans are 1-indexed
		card_index = (read_i32_from_stdin("Pick a card...".to_owned()) - 1) as usize;
		match players.get_from_current_player(card_index) {
			None => println!("Card does not exist!"),
			Some(x) => {
				// If wild, ask for color
				use cards::CardType::*;
				match x.card_type {
					Wild(mut color) | WildPlus4(mut color) => color = read_color_from_stdin(),
					_ => {},
				}
				return (*x, card_index);
			},		
		}
	}
}

fn read_i32_from_stdin(message: String) -> i32 {
	println!("{}", message);
	loop {
		let mut input = String::new();
		let result = io::stdin().read_line(&mut input);
		input.pop(); // Remove trailing newline
		match result {
			Ok(_) => {
				match input.parse::<i32>() {
					Ok(n) => return n,
					Err(_) => println!("{:?} is not a number!", input),
				}
			}
			Err(_) => panic!(),
		}
	}

}