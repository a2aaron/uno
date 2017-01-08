extern crate term_painter as term;

use self::term::ToStyle;

use game_state::{GameState, playable_card};
use cards::{Card, Color};

pub fn print_game_state(game_state: &GameState) {
	let top_card = game_state.top_card();
	println!("Top card is {}", color_card(top_card).paint(format!("{}", top_card)));
	println!("Your turn player {}!", game_state.players.current_player + 1);
	println!("Your hand");
	for (i, card) in game_state.players.get_current_player().iter().enumerate() {
		println!("[{}]: {}", i + 1, underline_if_playable(card, top_card));
	}
}

pub fn underline_if_playable(card: &Card, onto: &Card) -> term::Painted<String> {
	if playable_card(card, onto) {
		return color_card(card).underline().paint(format!("{}", card));
	} else {
		return color_card(card).paint(format!("{}", card));
	}
}

pub fn color_card(card: &Card) -> term::Color {
	use cards::CardType::*;
	match card.card_type {
		Wild(x) | WildPlus4(x) => return color_to_term_color(&x),
		_ => return color_to_term_color(&card.color),
	}
}


pub fn color_to_term_color(color: &Color) -> term::Color {
    use cards::Color::*;
    use self::term::Color as T_Color;
    match *color {
        Green => return T_Color::Green,
        Blue => return T_Color::Blue,
        Red => return T_Color::Red,
        Yellow => return T_Color::Yellow,
        Any =>  return T_Color::White,
    }
}