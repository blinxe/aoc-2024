use crate::utils::input::read_input;
#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
#[derive(Clone, Copy)]
struct Velocity {
    x: isize,
    y: isize,
}
type Action = char;
type Grid = Vec<Vec<char>>;

impl Pos {
    const ZERO: Self = Self { x: 0, y: 0 };

    fn add_velocity(&self, velocity: &Velocity) -> Pos {
        Pos {
            x: (self.x as isize + velocity.x) as usize,
            y: (self.y as isize + velocity.y) as usize,
        }
    }

    fn gps(&self) -> usize {
        100 * self.y + self.x
    }
}

impl Velocity {
    const UP: Self = Self { x: 0, y: -1 };
    const DOWN: Self = Self { x: 0, y: 1 };
    const LEFT: Self = Self { x: -1, y: 0 };
    const RIGHT: Self = Self { x: 1, y: 0 };

    fn from(a: Action) -> Self {
        match a {
            '^' => Self::UP,
            'v' => Self::DOWN,
            '<' => Self::LEFT,
            '>' => Self::RIGHT,
            _ => panic!("Invalid action '{}'", a),
        }
    }
}

fn display_grid(grid: &Grid, robot_pos: Pos) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let pos = Pos { x, y };
            if pos == robot_pos {
                print!("@");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> (Grid, Vec<Action>) {
    let mut split = input.split("\n\n");
    let input_part_1 = split.next().unwrap();
    let input_part_2 = split.next().unwrap();

    let grid = input_part_1.lines().map(|l| l.chars().collect()).collect();
    let actions = input_part_2.chars().filter(|c| *c != '\n').collect();

    (grid, actions)
}

fn try_move_robot(grid: &mut Grid, robot_pos: Pos, dir: Action) -> Pos {
    let mut new_robot_pos = robot_pos;
    let move_dir = Velocity::from(dir);
    let mut cursor = robot_pos.add_velocity(&move_dir);
    loop {
        match grid[cursor.y][cursor.x] {
            'O' => (),
            '.' => {
                new_robot_pos = robot_pos.add_velocity(&move_dir);
                if grid[new_robot_pos.y][new_robot_pos.x] == 'O' {
                    grid[cursor.y][cursor.x] = 'O';
                    grid[new_robot_pos.y][new_robot_pos.x] = '.';
                }
                break;
            }
            '#' => break,
            _ => panic!(),
        }
        cursor = cursor.add_velocity(&move_dir);
    }
    new_robot_pos
}

fn solve_part_1(input: &str) {
    let (mut grid, actions) = parse_input(input);
    let mut robot_pos = Pos::ZERO;
    'full_loop: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                robot_pos = Pos { x, y };
                grid[y][x] = '.';
                break 'full_loop;
            }
        }
    }
    for a in actions {
        robot_pos = try_move_robot(&mut grid, robot_pos, a);
    }
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                sum += Pos { x, y }.gps();
            }
        }
    }
    println!("Sum of GPS Coordinates {}", sum);
}

fn parse_input_v2(input: &str) -> (Grid, Vec<Action>) {
    let mut split = input.split("\n\n");
    let input_part_1 = split
        .next()
        .unwrap()
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.");
    let input_part_2 = split.next().unwrap();

    let grid = input_part_1.lines().map(|l| l.chars().collect()).collect();
    let actions = input_part_2.chars().filter(|c| *c != '\n').collect();
    (grid, actions)
}

fn can_move_box(grid: &Grid, box_pos: Pos, v: Velocity) -> bool {
    match v {
        Velocity { x: 0, .. } => {
            let v_left = box_pos.add_velocity(&v);
            let v_right = box_pos.add_velocity(&Velocity::RIGHT).add_velocity(&v);
            let ok_left = match grid[v_left.y][v_left.x] {
                '.' => true,
                '#' => false,
                '[' => can_move_box(grid, v_left, v),
                ']' => can_move_box(grid, v_left.add_velocity(&Velocity::LEFT), v),
                _ => panic!(),
            };
            let ok_right = match grid[v_right.y][v_right.x] {
                '.' => true,
                '#' => false,
                '[' => can_move_box(grid, v_right, v),
                ']' => true,
                _ => panic!(),
            };
            ok_left && ok_right
        }
        _ => {
            let mut neighboor = box_pos.add_velocity(&v);
            if grid[neighboor.y][neighboor.x] == ']' {
                neighboor = neighboor.add_velocity(&v);
            }
            match grid[neighboor.y][neighboor.x] {
                '[' => can_move_box(grid, neighboor, v),
                '.' => true,
                '#' => false,
                _ => panic!(),
            }
        }
    }
}

fn move_box(grid: &mut Grid, box_pos: Pos, v: Velocity) -> () {
    match v {
        Velocity { x: 0, .. } => {
            let v_left = box_pos.add_velocity(&v);
            let v_right = box_pos.add_velocity(&Velocity::RIGHT).add_velocity(&v);
            match grid[v_left.y][v_left.x] {
                '.' => (),
                '[' => move_box(grid, v_left, v),
                ']' => move_box(grid, v_left.add_velocity(&Velocity::LEFT), v),
                _ => panic!(),
            };
            match grid[v_right.y][v_right.x] {
                '.' => (),
                '[' => move_box(grid, v_right, v),
                _ => panic!(),
            };
            grid[v_left.y][v_left.x] = '[';
            grid[v_right.y][v_right.x] = ']';
            grid[box_pos.y][box_pos.x] = '.';
            grid[box_pos.y][box_pos.x + 1] = '.';
        }
        _ => {
            let mut neighboor = box_pos.add_velocity(&v);
            if grid[neighboor.y][neighboor.x] == ']' {
                neighboor = neighboor.add_velocity(&v);
            }
            match grid[neighboor.y][neighboor.x] {
                '[' => move_box(grid, neighboor, v),
                '.' => (),
                _ => panic!(),
            }
            grid[box_pos.y][(box_pos.x as isize + v.x) as usize] = '[';
            grid[box_pos.y][(box_pos.x as isize + v.x + 1) as usize] = ']';
            if v.x > 0 {
                grid[box_pos.y][box_pos.x] = '.';
            } else {
                grid[box_pos.y][box_pos.x + 1] = '.';
            }
        }
    }
}

fn try_move_robot_v2(grid: &mut Grid, robot_pos: Pos, dir: Action) -> Pos {
    let mut new_robot_pos = robot_pos;
    let move_dir = Velocity::from(dir);
    let cursor = robot_pos.add_velocity(&move_dir);
    if match grid[cursor.y][cursor.x] {
        '[' => {
            if can_move_box(grid, cursor, move_dir) {
                move_box(grid, cursor, move_dir);
                true
            } else {
                false
            }
        }
        ']' => {
            if can_move_box(
                grid,
                cursor.add_velocity(&Velocity { x: -1, y: 0 }),
                move_dir,
            ) {
                move_box(
                    grid,
                    cursor.add_velocity(&Velocity { x: -1, y: 0 }),
                    move_dir,
                );
                true
            } else {
                false
            }
        }
        '.' => true,
        '#' => false,
        _ => panic!(),
    } {
        new_robot_pos = cursor;
    }
    new_robot_pos
}

fn solve_part_2(input: &str) {
    let (mut grid, actions) = parse_input_v2(input);

    let mut robot_pos = Pos::ZERO;
    'full_loop: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                robot_pos = Pos { x, y };
                grid[y][x] = '.';
                break 'full_loop;
            }
        }
    }
    for a in actions {
        robot_pos = try_move_robot_v2(&mut grid, robot_pos, a);
    }
    display_grid(&grid, robot_pos);
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '[' {
                sum += Pos { x, y }.gps();
            }
        }
    }
    println!("Sum of GPS Coordinates {}", sum);
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
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    // const EXAMPLE_2: &str = indoc! {"
    //     #######
    //     #...#.#
    //     #.....#
    //     #..OO@#
    //     #..O..#
    //     #.....#
    //     #######

    //     <vv<<^^<<^^
    // "};
    const EXAMPLE_2: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
