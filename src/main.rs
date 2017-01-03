mod cards;
mod tests;
mod game_state;
mod colorize;

use cards::*;
use game_state::*;

use std::io;

fn main() {
	let mut num_players: usize = 4;
	println!("Welcome to Uno!");
	println!("Type \"start\" to play. Type a number to set number of players (Currently {})", num_players);
	// Preamble/Options
	loop {
		let menu_nav: String = read_string_from_stdin(None).to_lowercase();
		if menu_nav == "start" {
			break;
		} else if let Ok(n) = menu_nav.parse::<i32>() {
			if n <= 0 {
				println!("Must have at least one player");
			}
			else {
				num_players = n as usize;
				println!("Number of players is now {}", num_players);
			}
		} else {
			println!("Command not recoginized");
		}
	}

	println!("Uno game start! (Number of players: {})", num_players);
	let mut game_state: GameState = GameState::new(num_players);

	// Main game loop
	loop {
		colorize::print_game_state(&mut game_state);
		// Get card to play
		let action: Action = read_action_from_stdin(&mut game_state.players);
		match action {
			Action::Play(mut card, index) => {
				if game_state.playable_card(card) {
					game_state.players.get_current_player().remove(index);
					if game_state.players.get_current_player().len() == 0 {
						break;
					}
					game_state.play_card(&mut card);
				}
				else {
					println!("Cannot play {} onto {}", card, game_state.top_card());
				}
			},
			Action::Draw => {
				game_state.draw_card()
			}
		}
	}

	println!("You win player {}!", game_state.players.current_player + 1);
}

enum Action {
	Play(Card, usize),
	Draw,
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

fn read_action_from_stdin<'a>(players:&'a mut Players) -> Action {
	loop {
		let input = read_string_from_stdin(Some("Pick a card...".to_owned()));
		if let Ok(n) = input.parse::<usize>() {
			if n == 0 {
				println!("Card does not exist!");
				continue;
			}

			// Minus 1 because humans are 1-indexed
			let card_index: usize = n - 1 as usize;
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

fn read_string_from_stdin(message: Option<String>) -> String {
	if let Some(x) = message {
		println!("{}", x);
	}
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	input.pop(); // Remove trailing newline
	input
}