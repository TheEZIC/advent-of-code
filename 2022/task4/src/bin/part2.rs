use std::collections::HashMap;
use task4::{read_input, read_card_set};

fn main() {
    let input = read_input();
    process_part2(input);
}

fn process_part2(lines: Vec<String>) {
    let mut cards_points = HashMap::<u32, u32>::new();

    for line in lines.iter() {
        let (card_chunk, cards_sets) = line.split_once(":").unwrap();

        let card_chunks = card_chunk.split(" ")
            .filter(|str| !str.is_empty())
            .collect::<Vec<&str>>();

        let card_id = card_chunks[1].parse::<u32>().unwrap();

        let (winning_cards_string, player_cards_string) = cards_sets.split_once("|").unwrap();

        let winning_cards = read_card_set(winning_cards_string.trim().to_string());
        let player_cards = read_card_set(player_cards_string.trim().to_string());

        let mut winning_cards_count = 0;

        for card in player_cards {
            if winning_cards.contains(&card) {
                winning_cards_count += 1;
            }
        }

        for i in card_id..(card_id + winning_cards_count + 1) {
            if i > lines.len() as u32 {
                break;
            }

            let existed = match cards_points.get(&i) {
                Some(value) => *value,
                None => 0,
            };

            if i == card_id {
                cards_points.insert(i, existed + 1);
            } else {
                let current = cards_points.get(&card_id).unwrap();
                cards_points.insert(i, existed + current);
            }
        }
    }

    let mut points = 0;

    for value in cards_points.into_values() {
        points += value;
    }

    println!("{}", points);
}
