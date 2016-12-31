extern crate rand;

pub fn playable_card(card: Card, onto: Card) -> bool {
    use self::Color::*;
    use self::CardType::*;
    if card.color == Any {
        true
    } else if card.color == onto.color || card.card_type == onto.card_type {
        true
    } else {
        match onto.card_type {
            Wild(x) | WildPlus4(x) => card.color == x,
            _ => false,
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
                use self::CardType::*;
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
        use self::TurnOrder::*;
        let mut game_state = GameState {
            turn_order: Normal,
            play_deck: Vec::new(),
            draw_deck: get_deck(),
            players: Players::new(num_players),
        };
        game_state.shuffle();
        
        for i in 0..num_players {
            for j in 0..STARTING_HAND_SIZE {
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
    }

    fn skip(&mut self) {
        self.next_player();
        self.next_player();
    }

    fn plus_n(&mut self, num_cards: usize) {
        self.next_player();
        {
            let ref mut cards: Vec<Card> = self.draw_deck.split_off(num_cards);
            let this_hand: &mut Vec<Card> =  self.players.get_current_player();
            this_hand.append(cards);
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

    fn refill(&mut self) {
        use cards::rand::Rng;

        if self.draw_deck.len() != 0 {
            panic!("Draw deck not empty");
        } 

        self.draw_deck.append(&mut self.play_deck);
    }

    fn shuffle(&mut self) {
        use cards::rand::Rng;
        rand::thread_rng().shuffle(&mut self.draw_deck.as_mut_slice());
    }

    pub fn play_card(&mut self, card: &mut Card) -> Result<&mut GameState, Card> {
        use self::CardType::*;
        if playable_card(*card, *self.play_deck.first().unwrap()) {
            self.play_deck.push(*card);
            match card.card_type {
                Reverse => self.reverse(),
                Skip => self.skip(),
                Plus2 => self.plus_n(2),
                WildPlus4(_) => self.plus_n(4),
                _ => {},
            }
            return Ok(self)
        } else {
            return Err(*card)
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
        let mut players: Vec<Vec<Card>> = Vec::new();
        for i in 0..num_players {
            players.push(Vec::new());
        }
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
        self.current_player = (1 + self.current_player) % self.players.len()
    }

    pub fn previous_player(&mut self) {
        self.current_player = (1 - self.current_player) % self.players.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnOrder {
    Normal,
    Reverse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub color: Color,
    pub card_type: CardType,
}

impl Card {
    pub fn new() -> Card {
        Card {
            color: Color::Red,
            card_type: CardType::Number(-1),
        }
    }

    fn new_from_any(color: Color, card_type: CardType) -> Card {
        Card {
            color: color,
            card_type: card_type,
        }
    }

    pub fn new_from(color: Color, card_type: CardType) -> Result<Card, & 'static str> {
        /*
        A card must satisfy the following rules
        1. If it a wild card, then it's color must be Any
        2. If it is not a wild card, then it's color must not be Any
        3. If it is a number, then it must between 0 and 9 inclusive
        */
        use self::Color::*;
        use self::CardType::*;
         match (color, card_type) {
             (Any, Wild(_)) | (Any, WildPlus4(_)) => return Ok(Card::new_from_any(color, card_type)),
             (_, Wild(_)) | (_, WildPlus4(_)) => return Err("Wild or WildPlus4 card must use Any color"),
             (Any, _) => return Err("Any color must only be used with Wild or WildPlus4 cardtype"),
             (_, Number(x)) if (x < 0 ||  x > 9) => return Err("Number cardtype must have value between one and ten"),
             (_, _) => return Ok(Card::new_from_any(color, card_type)),
         }
    }
}

impl Iterator for Card {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        use self::CardType::*;
        match self.card_type {
            Number(x) if x < 9 => self.card_type = Number(x + 1),
            Number(9) => self.card_type = Reverse,
            Reverse => self.card_type = Skip,
            Skip => self.card_type = Plus2,
            Plus2 => {
               self.card_type = Number(0);
               use self::Color::*;
               match self.color {
                    Red => self.color = Green,
                    Green => self.color = Blue,
                    Blue => self.color = Yellow,
                    Yellow => {
                        self.color = Any;
                        self.card_type = Wild(Color::Any);
                    }
                    Any => self.color = Any,
               }
               
            }
            Wild(_) => self.card_type = WildPlus4(Color::Any),
            WildPlus4(_) => return None,
            _ => panic!(),
        }
        Some(*self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
	Green,
	Blue,
	Red,
	Yellow,
    Any,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardType {
    Number(i32),
    Reverse,
    Skip,
    Plus2,
    Wild(Color),
    WildPlus4(Color),
}