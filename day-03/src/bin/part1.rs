fn main() {
    let input_lines = include_str!("./day3.txt");
    let output = part1(input_lines);
    dbg!(output);
}

fn part1(input_lines: &str) -> i32 {
    let grid: Vec<Vec<char>> = input_lines.lines().map(|line| line.chars().collect()).collect();

    let mut cs = std::collections::HashSet::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch.is_digit(10) || ch == '.' {
                continue;
            }
            for dr in (r as i32 - 1)..=(r as i32 + 1) {
                for dc in (c as i32 - 1)..=(c as i32 + 1) {
                    if dr < 0 || dc < 0 || (dr as usize) >= grid.len() || (dc as usize) >= grid[dr as usize].len() {
                        continue;
                    }
                    let mut dc = dc as usize;
                    if !grid[dr as usize][dc].is_digit(10) {
                        continue;
                    }
                    while dc > 0 && grid[dr as usize][dc - 1].is_digit(10) {
                        dc -= 1;
                    }
                    cs.insert((dr as usize, dc));
                }
            }
        }
    }

    let mut ns = Vec::new();

    for &(r, mut c) in &cs {
        let mut s = String::new();
        while c < grid[r].len() && grid[r][c].is_digit(10) {
            s.push(grid[r][c]);
            c += 1;
        }
        ns.push(s.parse::<i32>().unwrap());
    }

    ns.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("
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
        assert_eq!(result, 4361)
    }
}