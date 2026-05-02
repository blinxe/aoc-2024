use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

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

impl Direction {
    pub const ALL: [Direction; 4] = [North, South, East, West];

    fn reversed(&self) -> Self {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    fn turns(&self) -> [(Self, Cost); 3] {
        match self {
            North | South => [(*self, 1), (East, 1001), (West, 1001)],
            East | West => [(*self, 1), (North, 1001), (South, 1001)],
        }
    }
}

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
                y: self.y - 1,
            },
            South => Pos {
                x: self.x,
                y: self.y + 1,
            },
            East => Pos {
                x: self.x + 1,
                y: self.y,
            },
            West => Pos {
                x: self.x - 1,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Path {
    pos: Pos,
    dir: Direction,
    cost: Cost,
}

fn get_path_next_paths(grid: &Grid, p: &Path) -> Vec<Path> {
    let mut next_paths = Vec::new();

    for (dir, added_cost) in p.dir.turns() {
        let new_pos = p.pos.moved(dir);
        if grid[new_pos] == '.' {
            next_paths.push(Path {
                pos: new_pos,
                cost: p.cost + added_cost,
                dir,
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

fn map_shortest_path(grid: &Grid, start: Pos) -> HashMap<(Pos, Direction), Cost> {
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

    map
}

fn solve_part_1(input: &str) {
    let (grid, start, end) = parse_input(input);
    let solution = map_shortest_path(&grid, start);
    let min = Direction::ALL
        .iter()
        .filter_map(|d| solution.get(&(end, *d)))
        .min()
        .unwrap();
    println!("Score: {}", min);
}

fn get_parent_best_paths(solution: &HashMap<(Pos, Direction), Cost>, p: &Path) -> Vec<Path> {
    let mut paths = Vec::new();

    // parent of current p
    let pos = p.pos.moved(p.dir.reversed());
    // println!("Path: {:?} - parent: {:?}", p, pos);

    // which paths from parent allow moving to p with optimal cost
    for (dir, added_cost) in p.dir.turns() {
        if p.cost < added_cost {
            continue;
        }
        // println!(
        //     "looking for {:?}/{:?}, got {:?}",
        //     turn,
        //     p.cost - added_cost,
        //     solution.get(&(pos, turn))
        // );
        if solution
            .get(&(pos, dir))
            .is_some_and(|&cost| cost == p.cost - added_cost)
        {
            paths.push(Path {
                pos,
                dir,
                cost: p.cost - added_cost,
            });
        }
    }

    paths
}

fn visit_best(solution: &HashMap<(Pos, Direction), Cost>, end: Pos) -> HashSet<Pos> {
    let mut set = HashSet::new();

    // get min score at end
    let min = Direction::ALL
        .iter()
        .filter_map(|d| solution.get(&(end, *d)))
        .min()
        .unwrap();

    // keep any of the 4 possible paths that reach end with min score
    let mut paths: HashSet<_> = Direction::ALL
        .iter()
        .filter_map(|d| {
            if solution.get(&(end, *d))? == min {
                Some(Path {
                    pos: end,
                    dir: *d,
                    cost: *min,
                })
            } else {
                None
            }
        })
        .collect();

    // while we can go back
    while !paths.is_empty() {
        let mut prev_paths = HashSet::new();
        // for each of the optimal paths
        for p in paths.iter() {
            // mark its position
            set.insert(p.pos);
            // get its parent's optimal paths for next turn
            prev_paths.extend(get_parent_best_paths(solution, &p));
        }
        paths = prev_paths;
    }

    set
}

fn solve_part_2(input: &str) {
    let (grid, start, end) = parse_input(input);
    let solution = map_shortest_path(&grid, start);
    let best = visit_best(&solution, end);

    println!("Best paths tiles: {}", 1 + best.len()); // start not included
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
