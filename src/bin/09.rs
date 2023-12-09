advent_of_code::solution!(9);

pub fn last(nums: Vec<isize>, rhs: isize) -> isize {
    *nums.last().unwrap() + rhs
}

pub fn first(nums: Vec<isize>, rhs: isize) -> isize {
    *nums.first().unwrap() - rhs
}

pub fn part_one(input: &str) -> Option<u32> {
    history(input, last)
}

fn history(input: &str, which: fn(Vec<isize>, isize) -> isize) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let nums = line
            .split(' ')
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        score += diff_nums(nums, which);
    }

    Some(score as u32)
}

fn diff_nums(nums: Vec<isize>, which: fn(Vec<isize>, isize) -> isize) -> isize {
    let new_nums = nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<isize>>();
    if new_nums.iter().filter(|n| **n != 0).count() != 0 {
        which(nums, diff_nums(new_nums, which))
    } else {
        which(nums, 0)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    history(input, first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two_rows() {
        assert_eq!(diff_nums(vec![10, 13, 16, 21, 30, 45], first), 5);
        assert_eq!(diff_nums(vec![1, 3, 6, 10, 15, 21], first), 0);
        assert_eq!(diff_nums(vec![0, 3, 6, 9, 12, 15], first), -3);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
