use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Unknown,
}

struct Pipes {
    width: isize,
    height: isize,
    pipe_vec: Vec<Pipe>,
}

const ALL_DELTAS: [(isize, isize, Pipe, Pipe); 36] = [
    // West
    (-1, 0, Pipe::Horizontal, Pipe::Horizontal), // - -
    (-1, 0, Pipe::Horizontal, Pipe::NEBend),     // - L
    (-1, 0, Pipe::Horizontal, Pipe::SEBend),     // - F
    (-1, 0, Pipe::NWBend, Pipe::Horizontal),     // J -
    (-1, 0, Pipe::NWBend, Pipe::NEBend),         // J L
    (-1, 0, Pipe::NWBend, Pipe::SEBend),         // J F
    (-1, 0, Pipe::SWBend, Pipe::Horizontal),     // J -
    (-1, 0, Pipe::SWBend, Pipe::NEBend),         // J L
    (-1, 0, Pipe::SWBend, Pipe::SEBend),         // J F
    // East
    (1, 0, Pipe::Horizontal, Pipe::Horizontal),
    (1, 0, Pipe::Horizontal, Pipe::NWBend),
    (1, 0, Pipe::Horizontal, Pipe::SWBend),
    (1, 0, Pipe::NEBend, Pipe::Horizontal),
    (1, 0, Pipe::NEBend, Pipe::NWBend),
    (1, 0, Pipe::NEBend, Pipe::SWBend),
    (1, 0, Pipe::SEBend, Pipe::Horizontal),
    (1, 0, Pipe::SEBend, Pipe::NWBend),
    (1, 0, Pipe::SEBend, Pipe::SWBend),
    // North
    (0, -1, Pipe::Vertical, Pipe::Vertical),
    (0, -1, Pipe::Vertical, Pipe::SWBend),
    (0, -1, Pipe::Vertical, Pipe::SEBend),
    (0, -1, Pipe::NWBend, Pipe::Vertical),
    (0, -1, Pipe::NWBend, Pipe::SWBend),
    (0, -1, Pipe::NWBend, Pipe::SEBend),
    (0, -1, Pipe::NEBend, Pipe::Vertical),
    (0, -1, Pipe::NEBend, Pipe::SWBend),
    (0, -1, Pipe::NEBend, Pipe::SEBend),
    // South
    (0, 1, Pipe::Vertical, Pipe::Vertical),
    (0, 1, Pipe::Vertical, Pipe::NWBend),
    (0, 1, Pipe::Vertical, Pipe::NEBend),
    (0, 1, Pipe::SWBend, Pipe::Vertical),
    (0, 1, Pipe::SWBend, Pipe::NWBend),
    (0, 1, Pipe::SWBend, Pipe::NEBend),
    (0, 1, Pipe::SEBend, Pipe::Vertical),
    (0, 1, Pipe::SEBend, Pipe::NWBend),
    (0, 1, Pipe::SEBend, Pipe::NEBend),
];

impl Pipes {
    fn get(&self, pos1x: isize, pos1y: isize) -> Option<Pipe> {
        if pos1x < 0 || pos1x >= self.width || pos1y < 0 || pos1y >= self.height {
            return None;
        }

        Some(self.pipe_vec[pos1y as usize * self.width as usize + pos1x as usize])
    }

    fn get_connections(
        &self,
        startx: isize,
        starty: isize,
    ) -> Option<(isize, isize, isize, isize)> {
        let p = self.get(startx, starty)?;
        let mut matches = HashSet::new();

        for (dx, dy, in_pipe, out_pipe) in ALL_DELTAS.iter() {
            let (x, y) = (startx + dx, starty + dy);
            if *in_pipe != p && p != Pipe::Start {
                continue;
            }

            if let Some(pipe) = self.get(x, y) {
                if pipe == *out_pipe || pipe == Pipe::Start {
                    matches.insert((x, y));
                }
            }
        }
        if matches.len() == 2 {
            let mut m_iter = matches.iter();
            let m1 = m_iter.next().unwrap();
            let m2 = m_iter.next().unwrap();

            Some((m1.0, m1.1, m2.0, m2.1))
        } else {
            panic!("Pipe has {} matches", matches.len());
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    find_loop(input)
}

fn find_loop(input: &str) -> Option<u32> {
    let pipes = make_pipes(input);

    let (startx, starty) = find_start(&pipes)?;

    let (mut pos1x, mut pos1y, mut pos2x, mut pos2y) = pipes.get_connections(startx, starty)?;

    let mut visited = HashSet::<(isize, isize)>::new();
    let mut steps = 1;

    visited.insert((startx, starty));

    loop {
        let mut new_pos = HashSet::<(isize, isize)>::new();

        for (x, y) in [(pos1x, pos1y), (pos2x, pos2y)] {
            visited.insert((x, y));
            let (new_pos1x, new_pos1y, new_pos2x, new_pos2y) = pipes.get_connections(x, y)?;
            new_pos.insert((new_pos1x, new_pos1y));
            new_pos.insert((new_pos2x, new_pos2y));
        }

        steps += 1;

        let next = new_pos
            .difference(&visited)
            .collect::<Vec<&(isize, isize)>>();
        if next.len() < 2 {
            break;
        }

        if next.len() != 2 {
            panic!("Next has {} elements", next.len());
        }

        pos1x = next[0].0;
        pos1y = next[0].1;
        pos2x = next[1].0;
        pos2y = next[1].1;
    }

    Some(steps)
}

fn find_start(pipes: &Pipes) -> Option<(isize, isize)> {
    let start = pipes.pipe_vec.iter().position(|p| *p == Pipe::Start)? as isize;
    let startx = start % pipes.width;
    let starty = start / pipes.width;
    Some((startx, starty))
}

fn make_pipes(input: &str) -> Pipes {
    let width = input.lines().next().unwrap().trim().len() as isize;
    let height = input.lines().count() as isize;
    let pipe_vec = input
        .chars()
        .map(|c| match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NEBend,
            'J' => Pipe::NWBend,
            '7' => Pipe::SWBend,
            'F' => Pipe::SEBend,
            'S' => Pipe::Start,
            '.' => Pipe::Ground,
            _ => Pipe::Unknown,
        })
        .filter(|p| *p != Pipe::Unknown)
        .collect::<Vec<_>>();

    Pipes {
        width,
        height,
        pipe_vec,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let pipes = make_pipes(input);

    // Walk the pipes
    let (startx, starty) = find_start(&pipes)?;

    let mut visited = HashSet::<(isize, isize)>::new();
    let mut poly = Vec::<(isize, isize)>::new();

    let mut curx = startx;
    let mut cury = starty;

    poly.push((curx, cury));

    // Build a complex polygon
    loop {
        visited.insert((curx, cury));

        let (pos1x, pos1y, pos2x, pos2y) = pipes.get_connections(curx, cury)?;
        let mut new_pos = HashSet::<(isize, isize)>::new();

        new_pos.insert((pos1x, pos1y));
        new_pos.insert((pos2x, pos2y));

        let next = new_pos
            .difference(&visited)
            .collect::<Vec<&(isize, isize)>>();

        if next.is_empty() {
            break;
        }

        curx = next[0].0;
        cury = next[0].1;
        let new_pipe = pipes.get(curx, cury)?;
        if new_pipe != Pipe::Horizontal && new_pipe != Pipe::Vertical {
            poly.push((curx, cury));
        }
    }

    println!("There are {} points on the polygon", poly.len());

    let mut inside = 0;
    for x in 0..pipes.width {
        for y in 0..pipes.height {
            if is_inside(x, y, &poly) && !visited.contains(&(x, y)) {
                inside += 1;
            }
        }
    }

    Some(inside)
}

pub fn is_inside(x: isize, y: isize, poly: &Vec<(isize, isize)>) -> bool {
    let num = poly.len();
    let mut j = num - 1;
    let mut c = false;

    for i in 0..num {
        if (x == poly[i].0) && (y == poly[i].1) {
            // point is a corner
            return false;
        }
        if (poly[i].1 > y) != (poly[j].1 > y) {
            let slope = (x - poly[i].0) * (poly[j].1 - poly[i].1)
                - (poly[j].0 - poly[i].0) * (y - poly[i].1);

            if slope == 0 {
                // point is on boundary
                return false;
            }
            if (slope < 0) != (poly[j].1 < poly[i].1) {
                c = !c;
            }
        }
        j = i;
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));

        let example = "..F7.\n\
        .FJ|.\n\
        SJ.L7\n\
        |F--J\n\
        LJ...\n";

        let result = part_one(&example);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let example = "...........\n\
        .S-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L--J.L--J.\n\
        ...........";
        let result = part_two(example);
        assert_eq!(result, Some(4));

        let example = ".F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJS7F-7L7.\n\
        ....F-J..F7FJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...";

        let result = part_two(example);
        assert_eq!(result, Some(8));

        let example = "FF7FSF7F7F7F7F7F---7\n\
        L|LJ||||||||||||F--J\n\
        FL-7LJLJ||||||LJL-77\n\
        F--JF--7||LJLJ7F7FJ-\n\
        L---JF-JLJ.||-FJLJJ7\n\
        |F|F-JF---7F7-L7L|7|\n\
        |FFJF7L7F-JF7|JL---7\n\
        7-L-JL7||F7|L7F-7F7|\n\
        L.L7LFJ|||||FJL7||LJ\n\
        L7JLJL-JLJLJL--JLJ.L";

        let result = part_two(example);
        assert_eq!(result, Some(10));
    }
}
