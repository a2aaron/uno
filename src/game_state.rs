extern crate rand;

use cards::*;

const STARTING_HAND_SIZE: usize = 7;

/// A GameState holds all of the information needed for an Uno game_state
/// This includes the players (each with their deck, and the current player)
/// the deck of cards to play on, the deck of cards to draw from, and the turn turn
/// Note that players may have zero cards (which means they have won) in their hand
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub turn_order: TurnOrder,
    pub players: Players,
    draw_deck: Vec<Card>,
    play_deck: Vec<Card>,
}

impl GameState {
    /// Constructs a new GameState
    /// This shuffles the deck, deals cards to players,
    /// and ensures that the top card on play_deck is not a wild card
    pub fn new(num_players: usize) -> GameState {
        use game_state::TurnOrder::*;
        let mut game_state = GameState {
            turn_order: Normal,
            play_deck: Vec::new(),
            draw_deck: get_deck(),
            players: Players::new(num_players),
        };

        game_state.shuffle();

        // Deal cards
        for i in 0..num_players {
            for _ in 0..STARTING_HAND_SIZE {
                let card: Card = game_state.pop_draw_deck();
                game_state.players.get_hand(i).push(card);
            }
        }
        game_state.deal_first_play_card();

        game_state
    }

    /// Play a card onto the deck. If this card cannot be played then
    /// this function panics (Use `playable_card` before to ensure the card may be played)
    pub fn play_card(&mut self, card: &mut Card) -> &mut GameState {
        use cards::CardType::*;
        if playable_card(card, self.top_card()) {
            self.play_deck.push(*card);
            match card.card_type {
                Reverse => self.reverse(),
                Skip => self.skip(),
                Plus2 => self.plus_n(2),
                WildPlus4(_) => self.plus_n(4),
                _ => self.next_player(),
            }
            return self
        } else {
            panic!()
        }
    }

    /// Goes to the next player (This goes backwards if a reverse is in play)
    fn next_player(&mut self) {
        use self::TurnOrder::*;
        match self.turn_order {
            Normal => self.players.next_player(),
            Reverse => self.players.previous_player(),
        }
    }

    /// Draws a card from the draw_deck (Refilling it if need be)
    /// Then, goes to the next player
    pub fn draw_card(&mut self) {
        let card: Card = self.pop_draw_deck();
        self.players.get_current_player_mut().push(card);
        self.next_player();
    }

    /// Get the top card of the `play_deck`. If it's empty then refill it
    fn pop_draw_deck(&mut self) -> Card {
        match self.draw_deck.pop() {
            Some(card) => return card,
            None => {
                self.refill();
                return self.pop_draw_deck();
            }
        }
    }

    pub fn top_card(&self) -> &Card {
        return self.play_deck.last().expect("Expected at least one card in the play_deck");
    }

    fn deal_first_play_card(&mut self) {
        // Make sure top card is not a wild card
        loop {
            let card: Card = self.pop_draw_deck();
            self.play_deck.push(card);
            if self.top_card().color != Color::Any {
                break;
            }
        }
    }

    fn reverse(&mut self) {
        use self::TurnOrder::*;
        match self.turn_order {
            Normal => self.turn_order = Reverse,
            Reverse => self.turn_order = Normal,
        }
        self.next_player();
    }

    fn skip(&mut self) {
        self.next_player();
        self.next_player();
    }

    fn plus_n(&mut self, num_cards: usize) {
        self.next_player();
        {
            let mut cards: Vec<Card> = Vec::new();
            for _ in 0..num_cards {
                cards.push(self.pop_draw_deck());
            }
            let this_hand: &mut Vec<Card> = self.players.get_current_player_mut();
            this_hand.append(&mut cards);
        }
        self.next_player();
    }

    fn refill(&mut self) {
        if self.draw_deck.len() != 0 {
            panic!("Draw deck not empty");
        } 
        self.draw_deck.append(&mut self.play_deck);
        self.deal_first_play_card();
        println!("Refilled draw deck");
    }

    fn shuffle(&mut self) {
        use self::rand::Rng;
        rand::thread_rng().shuffle(&mut self.draw_deck.as_mut_slice());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnOrder {
    Normal,
    Reverse,
}

/// Holds each player's hand and the current player's index
/// Note that at least one player should exist
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Players {
    pub current_player: usize, // TODO: Remove this pub
    players: Vec<Vec<Card>>,
}

impl Players {
    fn new(num_players: usize) -> Players {
        let players: Vec<Vec<Card>> = vec!(Vec::new(); num_players);
        Players {
            players: players,
            current_player: 0,
        }
    }

    /// Get the nth hand
    fn get_hand(&mut self, index: usize) -> &mut Vec<Card> {
        self.players.get_mut(index).unwrap()
    }

    /// Get the current player's hand
    pub fn get_current_player(&self) -> &Vec<Card> {
        self.players.get(self.current_player).unwrap()
    }

    pub fn get_current_player_mut(&mut self) -> &mut Vec<Card> {
        self.players.get_mut(self.current_player).unwrap()
    }

    /// Get the nth card from the current player's hand
    pub fn get_from_current_player(&self, index: usize) -> Option<&Card> {
        self.get_current_player().get(index)
    }

        /// Get the nth card from the current player's hand
    pub fn get_from_current_player_mut(&mut self, index: usize) -> Option<&mut Card> {
        self.get_current_player_mut().get_mut(index)
    }

    fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len()
    }

    fn previous_player(&mut self) {
        if self.current_player == 0 {
            self.current_player = self.players.len() - 1;
        } else {
            self.current_player = self.current_player - 1;
        }
    }
}

pub enum Action {
    Play(Card, usize),
    Draw,
}

/// Returns true if at least one of the following is true:
/// 1. The color of the card is "Any" (and thus is a Wild card)
/// 2. The color or card type of the card matches the `onto` card_type
/// 3. The color of the card matches the onto card (if the onto card is a Wild)
pub fn playable_card(card: &Card, onto: &Card) -> bool {
    use cards::Color::*;
    use cards::CardType::*;
    if card.color == Any {
        return true
    } else if card.color == onto.color || card.card_type == onto.card_type {
        return true
    } else {
        match onto.card_type {
            Wild(x) | WildPlus4(x) => return card.color == x,
            _ => {
                return false
            },
        }
    }
}

/// Returns a Vector of 108 cards containing:
/// 4 `WildPlus4` cards
/// 4 `Wild` cards
/// 2 of each color for Numbers 1 through 9, Reverse, Skip, and Plus2
/// 1 of each color for Number 0
fn get_deck() -> Vec<Card> {
    let mut vec: Vec<Card> = Vec::new();
    let mut iter: Card = Card::new();
    loop {
        let card: Option<Card> = iter.next();
        match card {
            Some(card) => {
                use cards::CardType::*;
                match card.card_type {
                    Number(0) => {
                        vec.push(card); 
                    },
                    Number(x) if x > 0 => {
                        vec.push(card);
                        vec.push(card);
                    },
                    Number(_) => unreachable!(),
                    Reverse | Skip | Plus2 => {
                        vec.push(card);
                        vec.push(card);
                    },
                    Wild(_) | WildPlus4(_) => {
                        vec.push(card);
                        vec.push(card);
                        vec.push(card);
                        vec.push(card);
                    },
                }
            }
            None => return vec,
        }
    }
}