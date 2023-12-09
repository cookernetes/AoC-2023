use std::collections::HashMap;

fn main() {
    let input_lines = include_str!("./day08.txt");
    let output = part1(input_lines);
    dbg!(output);
}

fn part1(input_lines: &str) -> u32 {
    let parsed_lines: Vec<&str> = input_lines
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let dirs: &str = parsed_lines[0];
    let dirs_len = dirs.chars().count();

    for (i, l) in parsed_lines.iter().enumerate() {
        let split: Vec<_> = l.split(" = ").collect();
        
        if i == 0 {
            continue;
        }

        let tup_from_split: (&str, &str) = (&split[1][1..4], &split[1][6..9]);
        nodes.insert(split[0], tup_from_split);
    }

    let mut i = 0;
    let mut current = "AAA";
    loop {
        let dir = dirs.chars().nth(i % dirs_len).unwrap();

        match dir {
            'L' => current = nodes.get(current).unwrap().0,
            'R' => current = nodes.get(current).unwrap().1,
            _ => unreachable!()
        };

        i += 1;

        // End Loop Stuff
        if current == "ZZZ" {
            break;
        }
    }

    i as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
        ");
        assert_eq!(result, 6)
    }
}
