use std::collections::HashMap;

fn main() {
    let input_lines = include_str!("./day08.txt");
    let output = part2(input_lines);
    dbg!(output);
}

// * ------------------------------ SOLUTION ------------------------------
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut max = a;
    let mut min = b;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn part2(input_lines: &str) -> usize {
    // ------------------------------ PARSING ------------------------------
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

    
    // ------------------------------ LOGIC ------------------------------
    let starting_nodes: Vec<&str> = nodes
        .keys()
        .filter(|&k| k.ends_with("A"))
        .cloned()
        .collect();

    let mut pathlens: Vec<_> = Vec::new();

    for mut current in starting_nodes {
        let mut i = 0;
        loop {
            let dir = dirs.chars().nth(i % dirs_len).unwrap();
    
            match dir {
                'L' => current = nodes.get(current).unwrap().0,
                'R' => current = nodes.get(current).unwrap().1,
                _ => unreachable!()
            };
    
            i += 1;
    
            // End Loop Stuff
            if current.ends_with("Z") {
                break;
            }
        }

        pathlens.push(i);
    }

    pathlens.into_iter().fold(1, |mut acc, step| {
        acc = lcm(acc, step);
        acc
    })
}


// * ------------------------------ TEST WITH EXAMPLE ------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("
LR

11A = (11B, XXX)
22A = (22B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
        ");
        assert_eq!(result, 6)
    }
}
