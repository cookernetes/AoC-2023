fn main() {
    let input_lines = include_str!("./day2.txt");
    let output = part1(input_lines);
    dbg!(output);
}

// Total allowed for each type: 12 red, 13 green, 14 blue.
fn part1(input_lines: &str) -> u32 {
    let mut valid_games: Vec<u32> = vec![];

    for line in input_lines.lines().filter(|l| !l.trim().is_empty()) {
        let id_split: Vec<&str> = line.split(": ").collect();
        let game_id = id_split[0];

        let props: Vec<Vec<(i8, &str)>> = id_split[1].split("; ").map(|section| {
            section.split(", ").map(|raw_prop| {
                let split_prop: Vec<&str> = raw_prop.split(" ").collect();
                let n = split_prop[0].parse::<i8>().expect("Quantity is not a digit...?");
                let c = split_prop[1];

                let n = match c {
                    "blue" => if n > 14 { -1 } else { n },
                    "green" => if n > 13 { -1 } else { n },
                    "red" => if n > 12 { -1 } else { n },
                    &_ => -1
                };

                (n, c)
            }).collect::<Vec<(i8, &str)>>()
        }).collect();
        
        if !props.iter().any(|line| line.iter().any(|&(n, _)| n == -1)) { 
            valid_games.push(game_id.split(" ").last().unwrap().parse().unwrap())
        }

    }
    
    valid_games.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ");
        assert_eq!(result, 8)
    }
}