use regex::Regex;

fn main() {
    let input_lines = include_str!("./day4.txt");
    let output = part2(input_lines);
    dbg!(output);
}

#[derive(Debug)]
struct Game {
    id: u32,
    winners: Vec<u32>,
    results: Vec<u32>
}

fn parse_input(input_lines: &str) -> Vec<Game> {
    let rows: Vec<&str> = input_lines.lines().filter(|l| !l.trim().is_empty()).collect();
    
    rows.iter().map(|r| {
        let mut id_data_split = r.split(": ");
        
        let id_portion = id_data_split.next().unwrap();
        let data_portion = id_data_split.last().unwrap();

        let re = Regex::new(r"\d+").unwrap();
        let mut dps = data_portion.split("|");

        let winners_raw = dps.next().unwrap();
        let results_raw = dps.last().unwrap();

        let winners: Vec<u32> = re.find_iter(winners_raw)
            .filter_map(|d| d.as_str().parse().ok())
            .collect();

        let results: Vec<u32> = re.find_iter(results_raw)
            .filter_map(|d| d.as_str().parse().ok())
            .collect();

        Game {
            results,
            winners,
            id: id_portion.split(" ").last().unwrap().parse::<u32>().unwrap()
        }
    }).collect()
}

fn part2(input_lines: &str) -> u32 {
    let parsed_games: Vec<Game> = parse_input(input_lines);
    let mut game_counts = vec![1; parsed_games.len()]; // Start with 1 instance of each game
    let mut index = 0;

    while index < parsed_games.len() {
        let game = &parsed_games[index];
        let mut matches = 0;

        for result in &game.results {
            if game.winners.contains(result) {
                matches += 1;
            }
        }

        // Distribute copies to subsequent games
        for next_game_index in index + 1..(index + 1 + matches).min(parsed_games.len()) {
            game_counts[next_game_index] += game_counts[index];
        }

        index += 1;
    }

    game_counts.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ");
        assert_eq!(result, 30)
    }
}
