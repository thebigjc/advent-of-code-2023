advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for l in input.lines() {
        sum += score_line_1(l);
    }
    Some(sum)
}

fn score_line_1(l: &str) -> u32 {
    let mut first = 10;
    let mut last = 10000;

    for c in l.chars() {
        if c.is_ascii_digit() {
            let d = c.to_digit(10).unwrap();

            if first == 10 {
                first = d;
            }

            last = d;
        }
    }

    first * 10 + last
}

fn score_line_2(l: &str) -> u32 {
    let mut first = 10;
    let mut last = 0;

    for i in 0..l.len() {
        let s = &l[i..];
        let c = s.chars().next().unwrap();
        if c.is_ascii_digit() {
            let d = c.to_digit(10).unwrap();

            if first == 10 {
                first = d;
            }

            last = d;
        } else {
            let d = if s.starts_with("zero") {
                0
            } else if s.starts_with("one") {
                1
            } else if s.starts_with("two") {
                2
            } else if s.starts_with("three") {
                3
            } else if s.starts_with("four") {
                4
            } else if s.starts_with("five") {
                5
            } else if s.starts_with("six") {
                6
            } else if s.starts_with("seven") {
                7
            } else if s.starts_with("eight") {
                8
            } else if s.starts_with("nine") {
                9
            } else {
                continue;
            };
            if first == 10 {
                first = d;
            }

            last = d;
        }
    }

    first * 10 + last
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for l in input.lines() {
        sum += score_line_2(l);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_line_1() {
        assert_eq!(score_line_1("1abc2"), 12);
        assert_eq!(score_line_1("treb7uchet"), 77);
    }

    #[test]
    fn test_score_line_2() {
        assert_eq!(score_line_2("two1nine"), 29);
        assert_eq!(score_line_2("eightwothree"), 83);
        assert_eq!(score_line_2("abcone2threexyz"), 13);
        assert_eq!(score_line_2("xtwone3four"), 24);
        assert_eq!(score_line_2("4nineeightseven2"), 42);
        assert_eq!(score_line_2("zoneight234"), 14);
        assert_eq!(score_line_2("7pqrstsixteen"), 76);
        assert_eq!(score_line_2("zoneight"), 18);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10309));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
