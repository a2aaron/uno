mod cards;
mod tests;
mod game_state;

// extern crate rand;

// use rand::Rand;

use cards::*;
use game_state::*;
use std::io;

fn main() {
	let mut game_state: GameState = GameState::new(4);
	println!("GameState::new OK");
	// Main game loop
	loop {
		println!("Your turn player {}!", game_state.players.current_player);
		println!("Top card is {:?}", game_state.top_card());
		println!("Your hand");
		for card in game_state.players.get_current_player() {
			println!("{:?}", card);
		}
		// Get card to play
		let (mut card, index) = read_card_from_stdin(&mut game_state.players);
		// Play the card
		if game_state.playable_card(card) {
			game_state.players.get_current_player().remove(index);
			game_state.play_card(&mut card);
		}
		else {
			println!("Cannot play {:?}", card);
		}
	}
}

fn read_color_from_stdin() -> Color {
	loop {
		println!("What color (R/G/Y/B)?");
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
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
					Wild(_) => x.card_type = Wild(read_color_from_stdin()),
					WildPlus4(_) => x.card_type = WildPlus4(read_color_from_stdin()),
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
		io::stdin().read_line(&mut input).unwrap();
		input.pop(); // Remove trailing newline
		match input.parse::<i32>() {
			Ok(n) => return n,
			Err(_) => println!("{:?} is not a number!", input),
		}
	}
}