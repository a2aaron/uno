#[cfg(test)]
mod tests {
	use cards::*;

	macro_rules! panic_on_err {
		($result: expr) => {
			match $result {
				Ok(_) => {},
				Err(_) => panic!(),
			}
		}
	}

	macro_rules! panic_on_ok {
		($result: expr) => {
			match $result {
				Ok(_) => panic!(),
				Err(_) => {},
			}
		}
	}

	macro_rules! panic_on_any_err {
		($( $result:expr),* ) => {
			$(panic_on_err!($result);)*
		}
	}

	macro_rules! panic_on_any_ok {
		($( $result:expr),* ) => {
			$(panic_on_ok!($result);)*
		}
	}

	#[test]
	fn test_playable_card() {
		// Play same color onto another
		let card: Card = Card {color: Color::Red, card_type: CardType::Number(6)};
		let onto: Card = Card {color: Color::Red, card_type: CardType::Skip};
		assert!(playable_card(card, onto));

		// Play same card type onto another
		let card = Card {color: Color::Red, card_type: CardType::Skip};
		let onto = Card {color: Color::Blue, card_type: CardType::Skip};
		assert!(playable_card(card, onto));

		// Play a wild card onto another
		let card = Card {color: Color::Any, card_type: CardType::Wild(Color::Any)};
		let onto = Card {color: Color::Blue, card_type: CardType::Skip};
		assert!(playable_card(card, onto));

		// Play a card onto another wild card of the same color
		let card = Card {color: Color::Red, card_type: CardType::Reverse};
		let onto = Card {color: Color::Any, card_type: CardType::WildPlus4(Color::Red)};
		assert!(playable_card(card, onto));

		// Cannot play a card onto another wild card of a different color
		let card = Card {color: Color::Red, card_type: CardType::Reverse};
		let onto = Card {color: Color::Any, card_type: CardType::WildPlus4(Color::Blue)};
		assert!(!playable_card(card, onto));

		// Cannot play a card that shares nothing with another card
		let card = Card {color: Color::Red, card_type: CardType::Number(6)};
		let onto = Card {color: Color::Blue, card_type: CardType::Skip};
		assert!(!playable_card(card, onto));
	}

	#[test]
	fn test_new_card() {
		panic_on_any_err!(
			Card::new_from(Color::Any, CardType::Wild(Color::Any)),
			Card::new_from(Color::Blue, CardType::Reverse),
			Card::new_from(Color::Green, CardType::Number(9)),
			Card::new_from(Color::Red, CardType::Number(0)),
			Card::new_from(Color::Any, CardType::WildPlus4(Color::Green))
		);
	}

	#[test]
	fn test_invalid_cards() {
		panic_on_any_ok!(
			Card::new_from(Color::Red, CardType::Wild(Color::Any)),
			Card::new_from(Color::Any, CardType::Number(2)),
			Card::new_from(Color::Red, CardType::Number(-1)),
			Card::new_from(Color::Red, CardType::Number(10))
		);
	}

	#[test]
	fn test_game_state() {
		let mut game_state: GameState = GameState {
 	   		turn_order: TurnOrder::Normal,
	    	current_player: 0,
	    	players: Vec::new(),
	    	draw_deck: Vec::new(),
    		play_deck: Vec::new(),
    	};

	}

	#[test]
	fn test_new_game_state() {
		let mut game_state: GameState = GameState::new_game();
		assert!(game_state.draw_deck == get_deck())
	}
}
