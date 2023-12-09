advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    // Time:        55     82     64     90
    // Distance:   246   1441   1012   1111
    let mut lines = input.lines();
    let times = parse_numbers(lines.next().unwrap());
    let distances = parse_numbers(lines.next().unwrap());

    Some(
        times
            .iter()
            .zip(distances)
            .map(|(t, d)| ways_to_win(*t, d))
            .product::<usize>() as u32,
    )
}

fn ways_to_win(t: usize, d: usize) -> usize {
    let mut count = 0;
    for i in 1..t {
        let dist = i * (t - i);
        if dist > d {
            count += 1;
        }
    }
    count
}

fn parse_numbers(num_str: &str) -> Vec<usize> {
    num_str[10..]
        .trim()
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time_str = lines.next().unwrap()[10..].replace(' ', "");
    let distance_str = lines.next().unwrap()[10..].replace(' ', "");
    let time = time_str.parse::<usize>().unwrap();
    let distance = distance_str.parse::<usize>().unwrap();

    Some(ways_to_win(time, distance) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_ways_to_win_1() {
        assert_eq!(ways_to_win(7, 9), 4);
        assert_eq!(ways_to_win(15, 40), 8);
        assert_eq!(ways_to_win(30, 200), 9);
    }

    #[test]
    fn test_ways_to_win_2() {
        assert_eq!(ways_to_win(71530, 940200), 71503);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
