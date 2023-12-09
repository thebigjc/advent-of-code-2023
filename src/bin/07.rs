advent_of_code::solution!(7);

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, EnumIter, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,  // 11
    Queen, // 12
    King,  // 13
    Ace,   // 14
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

fn parse_hand_joker(hand_str: &str) -> Hand {
    let cards = hand_str.chars().map(char_to_card).collect_vec();
    let all_hands = Card::iter()
        .map(|card| {
            cards
                .iter()
                .map(|x| if *x == Card::Jack { card } else { *x })
                .collect_vec()
        })
        .collect_vec();

    let mut max_hand = all_hands.iter().map(build_hand).max().unwrap();

    max_hand.cards = cards
        .iter()
        .map(|card| {
            if *card == Card::Jack {
                Card::Joker
            } else {
                *card
            }
        })
        .collect_vec();

    max_hand
}

fn parse_hand(hand_str: &str) -> Hand {
    let cards = hand_str.chars().map(char_to_card).collect_vec();
    build_hand(&cards)
}

fn build_hand(cards: &Vec<Card>) -> Hand {
    let card_counts = cards.iter().counts();
    let mut hand_type = HandType::HighCard;
    let max = *card_counts.values().max().unwrap();

    if max == 5 {
        hand_type = HandType::FiveOfAKind;
    } else if max == 4 {
        hand_type = HandType::FourOfAKind;
    } else if max == 3 {
        if card_counts.len() == 2 {
            hand_type = HandType::FullHouse;
        } else {
            hand_type = HandType::ThreeOfAKind;
        }
    } else if max == 2 {
        if card_counts.len() == 3 {
            hand_type = HandType::TwoPair;
        } else {
            hand_type = HandType::OnePair;
        }
    }

    Hand {
        hand_type,
        cards: cards.clone(),
    }
}

fn char_to_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    score_hands(input, parse_hand)
}

fn score_hands(input: &str, parser: fn(&str) -> Hand) -> Option<u32> {
    let sorted_hands = input
        .lines()
        .map(|line| {
            let hand = parser(&line[0..5]);
            let bid = line[6..].trim().parse::<usize>().unwrap();

            (hand, bid)
        })
        .sorted()
        .collect_vec();

    let mut score = 0;
    for (i, h) in sorted_hands.iter().enumerate() {
        score += h.1 * (i + 1);
    }

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    score_hands(input, parse_hand_joker)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_hand("AAAAA").hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_hand("AA8AA").hand_type, HandType::FourOfAKind);
        assert_eq!(parse_hand("23332").hand_type, HandType::FullHouse);
        assert_eq!(parse_hand("TTT98").hand_type, HandType::ThreeOfAKind);
        assert_eq!(parse_hand("23432").hand_type, HandType::TwoPair);
        assert_eq!(parse_hand("A23A4").hand_type, HandType::OnePair);
        assert_eq!(parse_hand("23456").hand_type, HandType::HighCard);
    }

    #[test]
    fn test_parse_hand_joker() {
        assert_eq!(parse_hand_joker("AAAAJ").hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_hand_joker("AA8AJ").hand_type, HandType::FourOfAKind);
        assert_eq!(parse_hand_joker("KTJJT").hand_type, HandType::FourOfAKind);
        assert_eq!(parse_hand_joker("233J2").hand_type, HandType::FullHouse);
        assert_eq!(parse_hand_joker("TJT98").hand_type, HandType::ThreeOfAKind);
        assert_eq!(parse_hand_joker("23432").hand_type, HandType::TwoPair);
        assert_eq!(parse_hand_joker("A23J4").hand_type, HandType::OnePair);
        assert_eq!(parse_hand_joker("23456").hand_type, HandType::HighCard);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
