use task7::read_input;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = read_input();
    process_part2(input);
}

fn process_part2(lines: Vec<String>) {
    let mut hands: Vec<CardHand> = Vec::new();

    for line in lines {
        let hand = CardHand::from_string(line);
        hands.push(hand);
    }

    hands.sort_by(|a, b| a.cmp(b));

    let mut answer: u32 = 0;

    for (i, hand) in hands.iter().enumerate() {
        answer += hand.bid * (i as u32 + 1);
    }

    println!("{}", answer);
}

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

#[derive(Eq)]
pub struct CardHand {
    pub pure: String,
    pub cards: CardsMap,
    pub bid: u32,
}

pub type CardsMap = HashMap<char, u32>;

impl CardHand {
    pub fn from_string(str: String) -> CardHand {
        let mut cards: CardsMap = HashMap::new();
        let (cards_chunk, bid_string) = str.split_once(" ").unwrap();

        for card in cards_chunk.chars() {
            if !cards.contains_key(&card) {
                cards.insert(card, 0);
            }

            *cards.get_mut(&card).unwrap() += 1;
        }

        let bid = bid_string.parse::<u32>().unwrap();

        return CardHand::new(cards_chunk.to_string(), cards, bid);
    }

    pub fn new(pure: String, cards: CardsMap, bid: u32) -> CardHand {
        return CardHand {
            pure,
            cards,
            bid,
        };
    }

    fn get_type(&self) -> CardHandType {
        let jokers = match self.cards.get(&'J') {
            Some(joker) => *joker,
            None => 0,
        };

        let values: Vec<&u32> = self.cards.values().collect();

        return match **values.iter().max().unwrap() + jokers {
            5 => CardHandType::FiveOfKind,
            4 => CardHandType::FourOfKind,
            3 => if values.len() == 2 { CardHandType::FullHouse } else { CardHandType::ThreeOfKind },
            2 => if values.len() == 3 { CardHandType::TwoPair } else { CardHandType::OnePair },
            1 => CardHandType::HighCard,
            _ => panic!("Unknown CardHand type"),
        }
    }

    fn get_card_nominal(char: &char) -> u32 {
        return match char {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unknown card nominal"),
        }
    }
}

impl PartialOrd<Self> for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();

        if self_type > other_type {
            return Ordering::Greater;
        }

        if self_type < other_type {
            return Ordering::Less;
        }

        let self_chars: Vec<char> = self.pure.chars().collect();
        let other_chars: Vec<char> = other.pure.chars().collect();

        if self_chars.len() != other_chars.len() {
            panic!("Card strings should be equals")
        }

        for i in 0..self_chars.len() {
            let self_nominal = CardHand::get_card_nominal(&self_chars[i]);
            let other_nominal = CardHand::get_card_nominal(&other_chars[i]);

            if self_nominal > other_nominal {
                return Ordering::Greater;
            }

            if self_nominal < other_nominal {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

impl PartialEq<Self> for CardHand {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}