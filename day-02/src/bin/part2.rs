fn main() {
    let input_lines = include_str!("./day2.txt");
    let output = part2(input_lines);
    dbg!(output);
}

struct Prop {
    color: String,
    n: u32
}


fn part2(input_lines: &str) -> u32 {
    let colors = ["blue", "red", "green"];
    
    input_lines
        .lines()
        .filter(|l| !l.trim().is_empty()).map(|line| {
            let line_props: Vec<Prop> = line.split(": ").last().unwrap().split("; ").flat_map(|section| {
                section.split(", ").map(|raw_prop| {
                    let split_prop: Vec<&str> = raw_prop.split(" ").collect();
                    Prop { 
                        color: split_prop[1].to_string(), 
                        n: split_prop[0].parse().unwrap()
                    }
                })
            }).collect();

            colors.iter().map(|&color| {
                line_props.iter()
                    .filter(|prop| prop.color == color)
                    .max_by_key(|prop| prop.n)
                    .map_or(0, |prop| prop.n)
            }).product::<u32>()
        }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ");
        assert_eq!(result, 2286)
    }
}