pub fn play_card(card: Card, mut deck: Vec<Card>) -> Option<Vec<Card>> {
    let top_card: Card = deck[deck.len() - 1];

    if playable_card(card, top_card) {
        deck.push(card);
        Some(deck)
    } else {
        None
    }
}

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

pub fn get_deck() -> [Card; 108] {
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
            None => break,
        }
    }
    let mut arr = [Card::new(); 108];
    for i in 0..arr.len() {
        arr[i] = vec[i];
    }

    arr
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
        4.
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