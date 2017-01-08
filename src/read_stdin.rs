use cards::Color;
use game_state::{Action, Players};
use std::io;


pub fn read_color_from_stdin() -> Color {
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

pub fn read_action_from_stdin<'a>(players:&'a mut Players) -> Action {
	loop {
		let input = read_string_from_stdin(Some("Pick a card...".to_owned()));
		if let Ok(n) = input.parse::<usize>() {
			if n == 0 {
				println!("Card does not exist!");
				continue;
			}

			// Minus 1 because humans are 1-indexed
			let card_index: usize = n - 1 as usize;
			match players.get_from_current_player_mut(card_index) {
				None => println!("Card does not exist!"),
				Some(x) => {
					// If wild, ask for color
					use cards::CardType::*;
					match x.card_type {
						Wild(_) => x.card_type = Wild(read_color_from_stdin()),
						WildPlus4(_) => x.card_type = WildPlus4(read_color_from_stdin()),
						_ => {},
					}
					return Action::Play(*x, card_index);
				},
			}	
		} else if input == "pass" || input == "p" {
			return Action::Draw;
		} else {
			println!("{:?} is not valid!", input);
		}
	}
}

pub fn read_string_from_stdin(message: Option<String>) -> String {
	if let Some(x) = message {
		println!("{}", x);
	}
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	input.pop(); // Remove trailing newline
	input
}