fn main() {
    let input_lines = include_str!("./day1.txt");
    let output = part1(input_lines);
    dbg!(output);
}


fn part1(input_lines: &str) -> u32 {
    let num_only_lines: Vec<String> = input_lines.split("\n").map(|line| 
        line.chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_string())
            .collect()
    ).filter(|l| l != "").collect();

    num_only_lines.iter()
        .map(|l| format!("{}{}", l.chars().nth(0).unwrap(), l.chars().last().unwrap()).parse::<u32>().unwrap())
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        ");
        assert_eq!(result, 142)
    }
}