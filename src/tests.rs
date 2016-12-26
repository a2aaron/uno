#[cfg(test)]
mod tests {
	use cards::*;

	#[test]
	fn test_playable_card() {
		let card: Card = Card {color: Color::Red, card_type: CardType::Number(6)};
		let onto: Card = Card {color: Color::Red, card_type: CardType::Skip};
		assert!(playable_card(card, onto));

		let card = Card {color: Color::Red, card_type: CardType::Skip};
		let onto = Card {color: Color::Blue, card_type: CardType::Skip};
		assert!(playable_card(card, onto));

		let card = Card {color: Color::Any, card_type: CardType::Wild(Color::Any)};
		let onto = Card {color: Color::Blue, card_type: CardType::Skip};
		assert!(playable_card(card, onto));

		let card = Card {color: Color::Red, card_type: CardType::Number(6)};
		let onto = Card {color: Color::Blue, card_type: CardType::Skip};
		assert!(!playable_card(card, onto));
	}

	#[test]
	fn test_new_card() {
		let card: Card = Card::new_from(Color::Any, CardType::Wild(Color::Any));
	}

	#[test]
	#[should_panic]
	fn test_invalid_card() {
		let card: Card = Card::new_from(Color::Red, CardType::Wild(Color::Any));
	}
}