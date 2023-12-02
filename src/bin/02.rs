advent_of_code::solution!(2);

pub fn limit(red: u32, green: u32, blue: u32, num: u32) -> u32 {
    if red > 12 || green > 13 || blue > 14 {
        0
    } else {
        num
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    score_game(input, limit)
}

pub fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn product(a: u32, b: u32, c: u32, _num: u32) -> u32 {
    a * b * c
}

pub fn part_two(input: &str) -> Option<u32> {
    score_game(input, product)
}

fn score_game(input: &str, eval: fn(u32, u32, u32, u32) -> u32) -> Option<u32> {
    let mut score = 0;

    for l in input.lines() {
        let colon = l.find(':')?;
        let num = l[5..colon].parse::<u32>().unwrap();
        let game_line = &l[colon + 2..];

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for subset in game_line.split("; ") {
            let cubes = subset.split(", ");
            for cube in cubes {
                let pick = cube.split(" ").collect::<Vec<&str>>();
                let n = pick[0].parse::<u32>().unwrap();
                let colour = pick[1];
                if colour == "red" {
                    red = max(red, n);
                } else if colour == "green" {
                    green = max(green, n);
                } else if colour == "blue" {
                    blue = max(blue, n);
                } else {
                    panic!();
                }
            }
        
        }
        score += eval(red, green, blue, num);
    }

    Some(score)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
