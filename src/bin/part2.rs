fn main() {
    let input_lines = include_str!("./day1.txt");
    let output = part2(input_lines);
    dbg!(output);
}

fn part2(input_lines: &str) -> u32 {
    input_lines.split('\n').filter(|l| !l.is_empty()).map(|line| {
        let replacements = [
            ("one", "o1e"), 
            ("two", "t2o"), 
            ("three", "t3e"), 
            ("four", "4"), 
            ("five", "5e"), 
            ("six", "6"), 
            ("seven", "7n"), 
            ("eight", "e8t"), 
            ("nine", "n9e")
        ];
    
        let modified_line = replacements.iter().fold(line.to_string(), |acc, &(from, to)| acc.replace(from, to));
    
        let mut digit_iter = modified_line.chars().filter(|&ch| ch.is_digit(10));
        let first_digit = digit_iter.next().unwrap_or_else(|| panic!("Expected a digit!"));
        let digit_combo = match digit_iter.last() {
            Some(last_digit) => format!("{first_digit}{last_digit}"),
            None => format!("{first_digit}{first_digit}"),
        };
        
        digit_combo.parse::<u32>().expect("Expected a number")
    }).sum::<u32>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_cases() {
        let result = part2("
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");
        assert_eq!(result, 281)
    }
}