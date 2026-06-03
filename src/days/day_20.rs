use std::collections::HashMap;

use crate::utils::input::read_input;

#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

type Grid = Vec<Vec<char>>;
type Dist = usize;
type DistMap = HashMap<Pos, Dist>;

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

// fn print_grid(grid: &Grid) {
//     for line in grid {
//         let s: String = line.iter().collect();
//         println!("{}", s);
//     }
// }

fn parse_input(input: &str) -> (Grid, Pos, Pos) {
    let mut grid: Grid = Vec::new();
    let mut start = Pos::default();
    let mut end = Pos::default();

    for (y, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            grid[y].push(match c {
                'S' => {
                    start = Pos::new(x, y);
                    '.'
                }
                'E' => {
                    end = Pos::new(x, y);
                    '.'
                }
                _ => c,
            });
        }
    }

    (grid, start, end)
}

fn find_distances(grid: &Grid, start: Pos) -> DistMap {
    let mut distmap = DistMap::new();

    let mut pos = start;
    let mut dist = 0;
    'browse: loop {
        distmap.insert(pos, dist);
        dist += 1;
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let nx = (pos.x as isize + dx) as usize;
            let ny = (pos.y as isize + dy) as usize;
            let neighbor = Pos::new(nx, ny);
            if grid[ny][nx] == '.' && !distmap.contains_key(&neighbor) {
                pos = neighbor;
                continue 'browse;
            }
        }
        break;
    }

    distmap
}

fn solve_part_1(input: &str) {
    let (grid, start, _end) = parse_input(input);
    // print_grid(&grid);
    // println!("start: {:?}", start);
    // println!("end: {:?}", _end);

    let distmap = find_distances(&grid, start);

    let mut cheats = Vec::new();
    for (pos, dist) in distmap.iter() {
        for (dx, dy) in [
            (0, -2),
            (0, 2),
            (-2, 0),
            (2, 0),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ] {
            let nx = (pos.x as isize + dx) as usize;
            let ny = (pos.y as isize + dy) as usize;
            let neighbor = Pos::new(nx, ny);
            if let Some(d) = distmap.get(&neighbor) {
                if d > dist {
                    let save = d - dist;
                    if save > 2 {
                        cheats.push((pos, neighbor, save - 2));
                    }
                }
            }
        }
    }

    // cheats.sort_by_key(|&(_, _, s)| s);
    // for c in cheats.iter() {
    //     println!("from {:?} to {:?}: {}", c.0, c.1, c.2);
    // }
    let good_saves = cheats.into_iter().filter(|&(_, _, s)| s >= 100).count();
    println!("Number of good cheats: {}", good_saves);
}

fn neighborhood(radius: isize) -> Vec<(isize, isize)> {
    let mut neighbors = Vec::new();
    for x in -radius..=radius {
        for y in -radius..=radius {
            let dist = x.abs() + y.abs();
            if dist >= 2 && dist <= radius {
                neighbors.push((x, y));
            }
        }
    }

    neighbors
}

fn solve_part_2(input: &str) {
    let (grid, start, _end) = parse_input(input);
    // print_grid(&grid);
    // println!("start: {:?}", start);
    // println!("end: {:?}", _end);

    let distmap = find_distances(&grid, start);
    let neighbors = neighborhood(20);

    // let instant = Instant::now();

    let mut cheats = Vec::new();
    for (pos, dist) in distmap.iter() {
        for (dx, dy) in neighbors.iter() {
            let nx = (pos.x as isize + dx) as usize;
            let ny = (pos.y as isize + dy) as usize;
            if nx >= grid[0].len() || ny >= grid.len() {
                continue;
            }
            if grid[ny][nx] == '#' {
                continue;
            }
            let neighbor = Pos::new(nx, ny);
            if let Some(d) = distmap.get(&neighbor) {
                if d > dist {
                    let save = d - dist;
                    if save > 2 {
                        cheats.push((pos, neighbor, save - (dx.abs() + dy.abs()) as usize));
                    }
                }
            }
        }
    }

    // cheats.sort_by_key(|&(_, _, s)| s);
    // for c in cheats.iter() {
    //     println!("from {:?} to {:?}: {}", c.0, c.1, c.2);
    // }
    // println!("found {} cheats in {:?}", cheats.len(), instant.elapsed());
    let good_saves = cheats.into_iter().filter(|&(_, _, s)| s >= 100).count();
    println!("Number of good cheats: {}", good_saves);
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
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
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
