use crate::utils::input::read_input;
use std::{ops::Range, usize};

type Id = usize;

fn parse_input(input: &str) -> (Vec<(Id, Range<u32>)>, Vec<Range<u32>>) {
    let mut files = Vec::new();
    let mut empties = Vec::new();
    let mut pos: u32 = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let length = c.to_digit(10).unwrap();
        let r = pos..pos + length;

        if i % 2 == 0 {
            files.push((i / 2, r));
        } else {
            empties.push(r);
        }
        pos += length;
    }
    (files, empties)
}

fn solve_part_1(input: &str) {
    let (files, empties) = parse_input(input);
    let mut new_files = Vec::new();
    let mut files_iter_rev = files.into_iter().rev();
    let mut last_file = files_iter_rev.next().unwrap();

    // build list of new files
    for mut e in empties {
        while e.len() > 0 {
            if last_file.1.start < e.end {
                break;
            }
            if last_file.1.len() > e.len() {
                // fill all empties
                new_files.push((last_file.0, e.clone()));
                last_file.1.end -= e.len() as u32;
            } else {
                // use full file and continue
                new_files.push((last_file.0, e.start..e.start + last_file.1.len() as u32));
                last_file = files_iter_rev.next().unwrap();
            }
            // reduce empty size by filled size
            e.start += new_files.last().unwrap().1.len() as u32;
        }
    }

    let mut result = 0;

    for f in new_files {
        result += f.0 * f.1.sum::<u32>() as usize;
    }
    for f in files_iter_rev {
        result += f.0 * f.1.sum::<u32>() as usize;
    }
    result += last_file.0 * last_file.1.sum::<u32>() as usize;

    println!("{:?}", result);
}

fn solve_part_2(input: &str) {
    let (files, mut empties) = parse_input(input);
    let mut result = 0;
    let mut new_files = Vec::new();

    for mut f in files.into_iter().rev() {
        for e in empties.iter_mut() {
            if e.start > f.1.start {
                new_files.push(f);
                break;
            }
            if e.len() >= f.1.len() {
                let len = f.1.len() as u32;
                f.1.start = e.start;
                f.1.end = e.start + len;
                new_files.push(f);

                e.start += len;
                break;
            }
        }
    }

    for f in new_files {
        result += f.0 * f.1.sum::<u32>() as usize;
    }
    println!("{:?}", result);
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

    const EXAMPLE_1: &str = indoc! {"2333133121414131402"};

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
