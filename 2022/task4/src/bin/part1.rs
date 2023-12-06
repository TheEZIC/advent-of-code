use task4::{read_input, read_card_set};

fn main() {
    let input = read_input();
    process_part1(input);
}

fn process_part1(lines: Vec<String>) {
    let mut points = 0;

    for line in lines.iter() {
        let (_, cards_sets) = line.split_once(":").unwrap();
        let (winning_cards_string, player_cards_string) = cards_sets.split_once("|").unwrap();

        let winning_cards = read_card_set(winning_cards_string.trim().to_string());
        let player_cards = read_card_set(player_cards_string.trim().to_string());

        let mut game_points: u32 = 0;

        for card in player_cards {
            if winning_cards.contains(&card) {
                if game_points == 0 {
                    game_points = 1;
                } else {
                    game_points *= 2;
                }
            }
        }

        points += game_points;
    }

    println!("{}", points);
}
