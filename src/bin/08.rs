use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn parse_line(line: &str) -> (&str, (&str, &str)) {
    // AAA = (BBB, CCC)
    let from = &line[0..3];
    let to_left = &line[7..10];
    let to_right = &line[12..15];

    (from, (to_left, to_right))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pattern, map) = parse_input(input);

    // Count steps
    let mut steps = 0;
    let mut cur = "AAA";

    while cur != "ZZZ" {
        let dir = pattern[steps % pattern.len()];
        cur = match dir {
            'L' => map[cur].0,
            'R' => map[cur].1,
            _ => panic!(),
        };
        steps += 1;
    }

    Some(steps as u32)
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let pattern = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let map: HashMap<&str, (&str, &str)> = lines.map(parse_line).collect();
    (pattern, map)
}

pub fn gcd(a1: usize, b1: usize) -> usize {
    let mut a = a1;
    let mut b = b1;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (pattern, map) = parse_input(input);

    let starts = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let mut all_steps = Vec::new();

    for start in starts {
        let mut cur = start;
        let mut steps = 0;
        while !cur.ends_with('Z') {
            let dir = pattern[steps % pattern.len()];
            cur = match dir {
                'L' => map[cur].0,
                'R' => map[cur].1,
                _ => panic!(),
            };
            steps += 1;
        }
        all_steps.push(steps);
    }

    let res = all_steps.clone().into_iter().reduce(lcm).unwrap();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));

        let other_example = "LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";

        let result = part_one(other_example);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let example = "LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)";
        let result = part_two(example);
        assert_eq!(result, Some(6));
    }
}
