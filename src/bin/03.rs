advent_of_code::solution!(3);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let line_len = input.lines().next()?.len();
    let input = input.replace('\n', "");
    let chars = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let mut i = 0;
    let mut parts = Vec::new();

    while i < chars.len() {
        let mut end = chars.len();
        if chars[i].is_ascii_digit() {
            for  (j, c) in chars.iter().enumerate().skip(i+1) {
                end = j;

                if j % line_len == 0 {
                    break;
                }
                if !c.is_ascii_digit() {
                    break;
                }
            }
            let adjacent = is_adjacent(i, end, line_len, &chars);
            println!("{}, {}", &input[i..end], adjacent);
            if adjacent {
                parts.push(input[i..end].parse::<u32>().unwrap());
            }
            i = end-1;
        }
        i += 1;
    }
    Some(parts.iter().sum())
}

fn is_adjacent(start: usize, end: usize, line_len: usize, chars: &[char]) -> bool {
    let lines = chars.len() / line_len;

    // Look left
    if start % line_len > 0 {
        let c = chars[start-1];
        if is_symbol(c) {
            return true;
        }
    }

    // Look Right
    if end % line_len < line_len - 1 {
        let c = chars[end];
        if is_symbol(c) {
            return true;
        }
    }

    // Look Up
    if look(false, start, line_len, end, lines, chars) {
        return true;
    }

    // Look Down
    if look(true, start, line_len, end, lines, chars) {
        return true;
    }

    false
}

fn look(up: bool, start: usize, line_len: usize, end: usize, lines: usize, chars: &[char]) -> bool {
    let mut x: i32 = (start % line_len) as i32;
    let mut y = start / line_len;
    
    if up {
        if y == 0 {
            return false;
        }
        y -= 1;
    } else {
        if y == lines - 1 {
            return false;
        }
        y += 1;
    }

    x -= 1;

    for i in 0..end-start+2 {
        if (x + (i as i32)) < 0 {
            continue;
        }

        let x_i = (x + (i as i32)) as usize;
        if x_i < line_len {
            let c = chars[x_i + y * line_len];
            if is_symbol(c) {
                return true;
            }
        }
    }
    false
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
