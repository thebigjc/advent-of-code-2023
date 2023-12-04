advent_of_code::solution!(4);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for l in input.lines() {
        let (_card, numbers) = split_card(l)?;
        let (winning_str, have_str) = numbers.split_once(" | ")?;
        let winning = parse_numbers(winning_str);
        let have = parse_numbers(have_str);
        let count = winning.intersection(&have).count();

        if count < 1 {
            continue;
        }

        sum += 1 << (count - 1);
    }

    Some(sum)
}

fn split_card(l: &str) -> Option<(usize, &str)> {
    let (card, numbers) = l.split_once(": ")?;
    Some((card[5..].trim().parse::<usize>().unwrap(), numbers))
}

fn parse_numbers(s: &str) -> HashSet<u32> {
    let have = s
        .split(' ')
        .filter(|n| !n.trim().is_empty())
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    have
}

struct Card {
    card: usize,
    count: usize,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = Vec::new();

    for l in input.lines() {
        let (card, numbers) = split_card(l).unwrap();
        let (winning_str, have_str) = numbers.split_once(" | ").unwrap();
        let winning = parse_numbers(winning_str);
        let have = parse_numbers(have_str);
        let count = winning.intersection(&have).count();

        let card = Card { card, count };
        cards.push(card);
    }

    let mut bonus = Vec::new();

    let mut score = cards.len() as u32;

    for card in &cards {
        for j in 0..card.count {
            bonus.push(card.card + j + 1);
        }
    }

    while !bonus.is_empty() {
        let card = &cards[bonus.pop().unwrap() - 1];
        for j in 0..card.count {
            bonus.push(card.card + j + 1);
        }
        score += 1;
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
