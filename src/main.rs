mod cards;
mod tests;
mod game_state;
mod colorize;
mod read_stdin;

use game_state::*;
use read_stdin::*;


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
		colorize::print_game_state(&game_state);
		// Get card to play
		let action: Action = read_action_from_stdin(&mut game_state.players);
		match action {
			Action::Play(mut card, index) => {
				if playable_card(&card, game_state.top_card()) {
					game_state.players.get_current_player_mut().remove(index);
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