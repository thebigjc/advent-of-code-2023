advent_of_code::solution!(3);

use itertools::Itertools;
use std::collections::HashSet;

type NumPoint = (usize, usize, usize, usize);

pub fn part_one(input: &str) -> Option<u32> {
    score_parts(input, is_symbol, part_one_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    score_parts(input, |c| c == '*', part_two_score)
}

fn score_parts(
    input: &str,
    f_symbol: fn(char) -> bool,
    scorer: fn(parts: HashSet<NumPoint>, input: String) -> u32,
) -> Option<u32> {
    let line_len = input.lines().next()?.len();
    let input = input.replace('\n', "");
    let chars = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let lines = chars.len() / line_len;
    let mut parts = HashSet::new();

    for (i, c) in chars.iter().enumerate() {
        if f_symbol(*c) {
            let x = i % line_len;
            let y = i / line_len;
            let deltas = [
                (-1, 0),  // Left
                (-1, -1), // Left, Up
                (0, -1),  // Up
                (1, -1),  // Right, Up
                (1, 0),   // Right
                (1, 1),   // Right, Down
                (0, 1),   // Down
                (-1, 1),  // Down Left
            ];
            for (dx, dy) in deltas.iter() {
                let x_i: i32 = x as i32 + dx;
                let y_i: i32 = y as i32 + dy;
                if x_i < 0 || y_i < 0 || x_i >= line_len as i32 || y_i >= lines as i32 {
                    continue;
                }
                let coords = find_number(x_i as usize, y_i as usize, line_len, &chars);
                match coords {
                    Some((a, b)) => {
                        parts.insert((x, y, a, b));
                    }
                    None => continue,
                }
            }
        }
    }

    Some(scorer(parts, input))
}

fn part_one_score(parts: HashSet<NumPoint>, input: String) -> u32 {
    let mut sum = 0;
    for part in parts.iter() {
        let (_x, _y, a, b) = part;
        sum += input[*a..*b].parse::<u32>().unwrap();
    }
    sum
}

fn part_two_score(parts: HashSet<NumPoint>, input: String) -> u32 {
    let mut parts_vec = parts.iter().collect::<Vec<&NumPoint>>();
    parts_vec.sort();
    parts_vec
        .iter()
        .group_by(|(x, y, _a, _b)| (x, y))
        .into_iter()
        .map(|(_k, v)| {
            let mut product = 1;
            let v = v.collect::<Vec<&&NumPoint>>();
            if v.len() == 1 {
                return 0;
            }

            for (_x, _y, a, b) in v {
                product *= input[*a..*b].parse::<u32>().unwrap();
            }

            product
        })
        .sum()
}

fn find_number(x: usize, y: usize, line_len: usize, chars: &[char]) -> Option<(usize, usize)> {
    let c = chars[x + y * line_len];
    if !c.is_ascii_digit() {
        return None;
    }

    // Look left until we run out of digits
    let mut start = x;
    while start > 0 {
        if !chars[start - 1 + y * line_len].is_ascii_digit() {
            break;
        }
        start -= 1;
    }

    // Look left until we run out of digits
    let mut end = x;
    while end < line_len - 1 {
        if !chars[end + 1 + y * line_len].is_ascii_digit() {
            break;
        }
        end += 1;
    }

    Some((start + y * line_len, end + 1 + y * line_len))
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
