extern crate rand;

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
        use cards::Color::*;
        use cards::CardType::*;
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
        use cards::CardType::*;
        match self.card_type {
            Number(x) if x < 9 => self.card_type = Number(x + 1),
            Number(9) => self.card_type = Reverse,
            Reverse => self.card_type = Skip,
            Skip => self.card_type = Plus2,
            Plus2 => {
               self.card_type = Number(0);
               use cards::Color::*;
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