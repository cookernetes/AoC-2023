use std::collections::HashSet;

fn main() {
    let input_lines = include_str!("./day3.txt");
    let output = part2(input_lines);
    dbg!(output);
}

fn part2(input_lines: &str) -> i32 {
    let grid: Vec<Vec<char>> = input_lines.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch != '*' {
                continue;
            }

            let mut cs = HashSet::new();

            for cr in (r as i32 - 1)..=(r as i32 + 1) {
                for cc in (c as i32 - 1)..=(c as i32 + 1) {
                    let (cr, mut cc) = (cr as usize, cc as usize);
                    if cr >= grid.len() || cc >= grid[cr].len() || !grid[cr][cc].is_digit(10) {
                        continue;
                    }
                    while cc > 0 && grid[cr][cc - 1].is_digit(10) {
                        cc -= 1;
                    }
                    cs.insert((cr, cc));
                }
            }

            if cs.len() != 2 {
                continue;
            }

            let mut ns: Vec<i32> = Vec::new();

            for &(cr, mut cc) in &cs {
                let mut s = String::new();
                while cc < grid[cr].len() && grid[cr][cc].is_digit(10) {
                    s.push(grid[cr][cc]);
                    cc += 1;
                }
                ns.push(s.parse::<i32>().unwrap());
            }

            total += ns[0] * ns[1];
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        ");
        assert_eq!(result, 467835)
    }
}