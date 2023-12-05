advent_of_code::solution!(5);

use core::ops::Range;

pub fn parse_seeds(input: &str) -> Vec<u64> {
    input[7..]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let seeds = parse_seeds(lines.next().unwrap());
    let stages = make_stages(lines);

    run_seeds(&stages, &seeds)
}

fn run_seeds(stages: &[Vec<(Range<u64>, Range<u64>)>; 7], seeds: &Vec<u64>) -> Option<u32> {
    let mut min = u64::MAX;

    for seed in seeds {
        let mut val = *seed;
        for stage in stages {
            val = map_stage(val, stage);
        }
        if val < min {
            min = val;
        }
    }

    Some(min as u32)
}

fn make_stages(mut lines: std::str::Lines<'_>) -> [Vec<(Range<u64>, Range<u64>)>; 7] {
    // Skip blank
    lines.next();

    let seed_to_soil = parse_section(&mut lines);

    let soil_to_fertilizer = parse_section(&mut lines);
    let fertilizer_to_water = parse_section(&mut lines);
    let water_to_light = parse_section(&mut lines);
    let light_to_temperature = parse_section(&mut lines);
    let temperature_to_humidity = parse_section(&mut lines);
    let humidity_to_location = parse_section(&mut lines);

    [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ]
}

fn map_stage(seed: u64, maps: &Vec<(Range<u64>, Range<u64>)>) -> u64 {
    let mut out = seed;
    for (dst, src) in maps {
        if src.contains(&seed) {
            out = dst.start + seed - src.start;
            break;
        }
    }

    out
}

fn parse_section(lines: &mut std::str::Lines<'_>) -> Vec<(Range<u64>, Range<u64>)> {
    // Skip header
    lines.next();

    let mut output = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let row: Vec<u64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        output.push((row[0]..row[0] + row[2], row[1]..row[1] + row[2]));
    }

    output
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let seeds = parse_seeds(lines.next().unwrap());
    let stages = make_stages(lines);

    println!("{:?}", seeds.len() / 2);

    let mut val: u32 = u32::MAX;
    for i in (0..seeds.len()).step_by(2) {
        let seed_range = seeds[i]..seeds[i] + seeds[i + 1];
        println!("{:?}", seed_range);

        let min = run_seeds(&stages, &seed_range.into_iter().collect::<Vec<u64>>())?;

        if min < val {
            val = min;
        }

        println!("{:?}, {:?}", min, val);
    }

    Some(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
