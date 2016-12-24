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
                use cards::$Name::*;
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

randable_struct! {
	pub struct Card {
		color: Color,
		card_type: CardType,
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

randable_enum! {
	pub enum CardType {
		One,
		Two,
		Three,
		Four,
		Five,
		Six,
		Seven,
		Eight,
		Nine,
		Ten,
		Skip,
		Reverse,
		Plus2,
		Wild,
		WildPlus4,
	}
}