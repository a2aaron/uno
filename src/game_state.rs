extern crate rand;

use cards::*;

pub fn playable_card(card: Card, onto: Card) -> bool {
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

pub fn get_deck() -> Vec<Card> {
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

const STARTING_HAND_SIZE: usize = 7;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub turn_order: TurnOrder,
    pub players: Players,
    pub draw_deck: Vec<Card>,
    pub play_deck: Vec<Card>,
}

impl GameState {
    pub fn new(num_players: usize) -> GameState {
        use game_state::TurnOrder::*;
        let mut game_state = GameState {
            turn_order: Normal,
            play_deck: Vec::new(),
            draw_deck: get_deck(),
            players: Players::new(num_players),
        };

        game_state.shuffle();
        for i in 0..num_players {
            for _ in 0..STARTING_HAND_SIZE {
                let card: Card = game_state.pop_draw_deck();
                game_state.players.get_hand(i).push(card);
            }
        }
        let card: Card = game_state.pop_draw_deck();
        game_state.play_deck.push(card);
        game_state
    }

    pub fn next_player(&mut self) {
        use self::TurnOrder::*;
        match self.turn_order {
            Normal => self.players.next_player(),
            Reverse => self.players.previous_player(),
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
            let this_hand: &mut Vec<Card> = self.players.get_current_player();
            this_hand.append(&mut cards);
        }
        self.next_player();
    }

    pub fn pop_draw_deck(&mut self) -> Card {
        match self.draw_deck.pop() {
            Some(card) => return card,
            None => {
                self.refill();
                return self.pop_draw_deck();
            }
        }
    }

    pub fn top_card(&mut self) -> Card {
        return *self.play_deck.last().unwrap();
    }

    fn refill(&mut self) {
        if self.draw_deck.len() != 0 {
            panic!("Draw deck not empty");
        } 

        self.draw_deck.append(&mut self.play_deck);
    }

    fn shuffle(&mut self) {
        use self::rand::Rng;
        rand::thread_rng().shuffle(&mut self.draw_deck.as_mut_slice());
    }

    pub fn playable_card(&mut self, card: Card) -> bool {
        return playable_card(card, self.top_card())
    }

    pub fn draw_card(&mut self) {
        let card: Card = self.pop_draw_deck();
        self.players.get_current_player().push(card);
        self.next_player();
    }

    pub fn play_card(&mut self, card: &mut Card) -> &mut GameState {
        use cards::CardType::*;
        if playable_card(*card, self.top_card()) {
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Players {
    pub current_player: usize,
    pub players: Vec<Vec<Card>>,    
}

impl Players {
    pub fn new(num_players: usize) -> Players {
        let players: Vec<Vec<Card>> = vec!(Vec::new(); num_players);
        Players {
            players: players,
            current_player: 0,
        }
    }

    pub fn get_hand(&mut self, index: usize) -> &mut Vec<Card> {
        self.players.get_mut(index).unwrap()
    }

    pub fn get_current_player(&mut self) -> &mut Vec<Card> {
        self.players.get_mut(self.current_player).unwrap()
    }

    pub fn get_from_current_player(&mut self, index: usize) -> Option<&mut Card> {
        self.get_current_player().get_mut(index)
    }

    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len()
    }

    pub fn previous_player(&mut self) {
        if self.current_player == 0 {
            self.current_player = self.players.len() - 1;
        } else {
            self.current_player = self.current_player - 1;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnOrder {
    Normal,
    Reverse,
}
