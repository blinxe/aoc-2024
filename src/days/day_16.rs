use std::{collections::HashMap, ops::Index};

use crate::utils::input::read_input;

type Coord = usize;
type Cost = usize;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: Coord,
    y: Coord,
}

impl Pos {
    const ZERO: Self = Pos { x: 0, y: 0 };

    fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    fn moved(&self, dir: Direction) -> Self {
        match dir {
            North => Pos {
                x: self.x,
                y: self.y.saturating_add_signed(-1),
            },
            South => Pos {
                x: self.x,
                y: self.y.saturating_add_signed(1),
            },
            East => Pos {
                x: self.x.saturating_add_signed(1),
                y: self.y,
            },
            West => Pos {
                x: self.x.saturating_add_signed(-1),
                y: self.y,
            },
        }
    }
}

impl Index<Pos> for Grid {
    type Output = char;
    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.y][index.x]
    }
}

#[derive(Debug, Clone, Copy)]
struct Path {
    pos: Pos,
    dir: Direction,
    cost: Cost,
}

fn get_path_next_paths(grid: &Grid, p: &Path) -> Vec<Path> {
    let turns = match p.dir {
        North | South => [East, West],
        East | West => [North, South],
    };

    let mut next_paths = Vec::new();

    let new_pos = p.pos.moved(p.dir);
    if grid[new_pos] == '.' {
        next_paths.push(Path {
            pos: new_pos,
            cost: p.cost + 1,
            dir: p.dir,
        });
    }

    for t in turns {
        let new_pos = p.pos.moved(t);
        if grid[new_pos] == '.' {
            next_paths.push(Path {
                pos: new_pos,
                cost: p.cost + 1001,
                dir: t,
            });
        }
    }

    next_paths
}

fn parse_input(input: &str) -> (Grid, Pos, Pos) {
    let mut grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = Pos::ZERO;
    let mut end = Pos::ZERO;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                'S' => start = Pos::new(x, y),
                'E' => {
                    end = Pos::new(x, y);
                    grid[y][x] = '.';
                }
                _ => (),
            }
        }
    }

    (grid, start, end)
}

fn solve_part_1(input: &str) {
    let (grid, start, end) = parse_input(input);

    let mut paths = vec![Path {
        pos: start,
        dir: East,
        cost: 0,
    }];

    let mut map = HashMap::<(Pos, Direction), Cost>::new();

    loop {
        let mut new_paths = Vec::new();
        for p in paths {
            for next in get_path_next_paths(&grid, &p) {
                let key = &(next.pos, next.dir);
                if !map.contains_key(key) {
                    map.insert(*key, next.cost);
                } else {
                    let prev_cost = map.get(key).unwrap();
                    if next.cost < *prev_cost {
                        map.insert(*key, next.cost);
                    } else {
                        continue;
                    } // don't keep this path for next step
                }
                new_paths.push(next);
            }
        }

        if new_paths.is_empty() {
            break;
        }
        paths = new_paths;
    }

    let min = [North, South, East, West]
        .iter()
        .filter_map(|d| map.get(&(end, *d)))
        .min()
        .unwrap();
    println!("Score: {}", min);
}

fn solve_part_2(input: &str) {}

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
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
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
