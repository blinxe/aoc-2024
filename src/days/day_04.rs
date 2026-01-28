use crate::utils::input::read_input;

type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn count_xmas_from(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0usize;
    'direction: for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let mut xx = x as isize;
        let mut yy = y as isize;
        for c in "MAS".chars() {
            xx += dx;
            yy += dy;
            if !(0isize..grid[0].len() as isize).contains(&xx)
                || !(0isize..grid.len() as isize).contains(&yy)
            {
                continue 'direction;
            }
            if c != grid[yy as usize][xx as usize] {
                continue 'direction;
            }
        }
        count += 1;
    }

    count
}

fn solve_part_1(input: &str) {
    let grid = parse_input(input);
    let mut count = 0usize;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 'X' {
                continue;
            }
            count += count_xmas_from(&grid, x, y);
        }
    }

    println!("Count: {}", count);
}

fn check_cross_mas_from(grid: &Grid, x: usize, y: usize) -> bool {
    let mut s = String::new();
    s.push(grid[y - 1][x - 1]);
    s.push(grid[y + 1][x + 1]);
    if !["MS", "SM"].contains(&s.as_str()) {
        return false;
    }
    let mut s = String::new();
    s.push(grid[y - 1][x + 1]);
    s.push(grid[y + 1][x - 1]);
    if !["MS", "SM"].contains(&s.as_str()) {
        return false;
    }

    return true;
}

fn solve_part_2(input: &str) {
    let grid = parse_input(input);
    let mut count = 0usize;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] != 'A' {
                continue;
            }
            if check_cross_mas_from(&grid, x, y) {
                count += 1;
            }
        }
    }

    println!("Count: {}", count);
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
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
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
