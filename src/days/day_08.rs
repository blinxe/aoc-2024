use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::input::read_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn is_in_map(&self, size: &Coord) -> bool {
        self.x >= 0 && self.x < size.x && self.y >= 0 && self.y < size.y
    }
}

fn parse_input(input: &str) -> (Coord, HashMap<char, Vec<Coord>>) {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            map.entry(c).or_insert(Vec::new()).push(Coord {
                x: x as isize,
                y: y as isize,
            });
        }
    }

    let size = Coord {
        x: input.lines().next().unwrap().len() as isize,
        y: input.lines().count() as isize,
    };
    (size, map)
}

fn find_antinodes(a1: &Coord, a2: &Coord) -> (Coord, Coord) {
    (
        Coord {
            x: 2 * a1.x - a2.x,
            y: 2 * a1.y - a2.y,
        },
        Coord {
            x: 2 * a2.x - a1.x,
            y: 2 * a2.y - a1.y,
        },
    )
}

fn solve_part_1(input: &str) {
    let (size, map) = parse_input(input);

    let mut antinodes = HashSet::new();
    for (k, v) in map {
        for c in v.iter().combinations(2) {
            let (an1, an2) = find_antinodes(c[0], c[1]);
            if an1.is_in_map(&size) {
                antinodes.insert(an1);
            }
            if an2.is_in_map(&size) {
                antinodes.insert(an2);
            }
        }
    }

    println!("Antinodes: {}", antinodes.len());
}

fn find_antinodes_v2(a1: &Coord, a2: &Coord, size: &Coord) -> Vec<Coord> {
    let dx = a2.x - a1.x;
    let dy = a2.y - a1.y;

    let mut v = Vec::new();

    let mut pos = Coord { x: a1.x, y: a1.y };
    while pos.is_in_map(size) {
        v.push(pos);
        pos.x -= dx;
        pos.y -= dy;
    }

    pos.x = a2.x;
    pos.y = a2.y;
    while pos.is_in_map(size) {
        v.push(pos);
        pos.x += dx;
        pos.y += dy;
    }

    v
}

fn solve_part_2(input: &str) {
    let (size, map) = parse_input(input);

    let mut antinodes = HashSet::new();
    for (k, v) in map {
        for c in v.iter().combinations(2) {
            antinodes.extend(find_antinodes_v2(c[0], c[1], &size));
        }
    }

    println!("Antinodes: {}", antinodes.len());
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str());
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
