use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialOrd, PartialEq)]
pub enum CardHandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

pub type CardsMap = HashMap<char, u32>;

pub trait AbstractCardHand {
    fn get_type(&self) -> CardHandType;

    fn get_card_nominal(&self, char: &char) -> u32;

    fn get_pure(&self) -> &String;
}

pub trait AbstractCardHandComparator: PartialOrd<Self> + PartialEq<Self> + Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();

        if self_type > other_type {
            return Ordering::Greater;
        }

        if self_type < other_type {
            return Ordering::Less;
        }

        let self_chars: Vec<char> = self.get_pure().chars().collect();
        let other_chars: Vec<char> = other.get_pure().chars().collect();

        if self_chars.len() != other_chars.len() {
            panic!("Card strings should be equals")
        }

        for i in 0..self_chars.len() {
            let self_nominal = self.get_card_nominal(&self_chars[i]);
            let other_nominal = self.get_card_nominal(&other_chars[i]);

            if self_nominal > other_nominal {
                return Ordering::Greater;
            }

            if self_nominal < other_nominal {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }

    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}