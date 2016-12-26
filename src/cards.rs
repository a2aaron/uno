extern crate rand;

use rand::{Rand, Rng};


macro_rules! count {
    () => { 0 };
    ($x:expr) => { 1 };
    ($x:expr, $($xs:expr),*) => { 1 + count!($($xs),*) }
}

macro_rules! randable_enum {
    (pub enum $Name:ident { $($x:ident,)* }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $Name {
            $($x,)*
        }

        impl $Name {
            fn values() -> [$Name; count!($($x),*)] {
                use self::$Name::*;
                [$($x),*]
            }
        }

        impl Rand for $Name {
            fn rand<R: Rng>(rng: &mut R) -> $Name {
                $Name::values()[rng.gen_range(0, $Name::values().len())]
            }
        }
    }
}

macro_rules! randable_struct {
    (pub struct $Name:ident {
         $($field_name:ident: $field_type:ty,)*
     }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $Name {
            $($field_name: $field_type,)*
        }

        impl Rand for $Name {
            fn rand<R: Rng>(rng: &mut R) -> $Name {
                $Name {
                    $($field_name: Rand::rand(rng),)*
                }
            }
        }
    }
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

randable_enum! {
	pub enum Color {
		Green,
		Blue,
		Red,
		Yellow,
		Any,
	}
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