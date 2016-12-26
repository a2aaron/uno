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
                    Number(_) => panic!("{:?}", card.card_type),
                    Reverse | Skip | Plus2 => {
                        vec.push(card);
                        vec.push(card);
                    },
                    Wild | WildPlus4 => {
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
    to_array(&vec)
}

fn to_array(vec: &Vec<Card>) -> [Card; 108] {
    let mut arr = [Card::new(); 108];
    for i in 0..arr.len() {
        arr[i] = vec[i];
    }

    arr
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    color: Color,
    card_type: CardType,
}

impl Card {
    pub fn new() -> Card {
        Card {
            color: Color::Red,
            card_type: CardType::Number(-1),
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
                        self.card_type = Wild;
                    }
                    Any => self.color = Any,
               }
               
            }
            Wild => self.card_type = WildPlus4,
            WildPlus4 => return None,
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
    Wild,
    WildPlus4,
}