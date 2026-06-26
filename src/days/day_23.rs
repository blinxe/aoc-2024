use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use itertools::Itertools;
use memoize::memoize;

use crate::utils::input::read_input;

fn computer_name_to_u16(name: &str) -> u16 {
    ((name.bytes().nth(0).unwrap() as u16) << 8) | (name.bytes().nth(1).unwrap() as u16)
}
fn computer_u16_to_name(value: u16) -> String {
    let result = (((value >> 8) as u8) as char).to_string() + &((value as u8) as char).to_string();
    // println!("{} => {}", value, result);
    result
}

fn starts_with_t(c: u16) -> bool {
    (c >> 8) == 't' as u16
}

fn parse_input(input: &str) -> HashMap<u16, HashSet<u16>> {
    let mut map = HashMap::new();
    for l in input.lines() {
        let mut parts = l.split('-');
        let p1 = computer_name_to_u16(parts.next().unwrap());
        let p2 = computer_name_to_u16(parts.next().unwrap());
        if !map.contains_key(&p1) {
            map.insert(p1, HashSet::new());
        }
        map.get_mut(&p1).unwrap().insert(p2);
        if !map.contains_key(&p2) {
            map.insert(p2, HashSet::new());
        }
        map.get_mut(&p2).unwrap().insert(p1);
    }
    map // list of connections, per computer
}

fn parse_input_v2(input: &str) -> HashMap<u16, Vec<u16>> {
    let mut map = HashMap::new();
    for l in input.lines() {
        let mut parts = l.split('-');
        let p1 = computer_name_to_u16(parts.next().unwrap());
        let p2 = computer_name_to_u16(parts.next().unwrap());
        if !map.contains_key(&p1) {
            map.insert(p1.clone(), Vec::new());
        }
        map.get_mut(&p1).unwrap().push(p2);
        if !map.contains_key(&p2) {
            map.insert(p2.clone(), Vec::new());
        }
        map.get_mut(&p2).unwrap().push(p1);
    }

    for v in map.values_mut() {
        v.sort();
    }
    map // list of connections, per computer
}

fn solve_part_1(input: &str) {
    let connections = parse_input(input);
    // println!("{:?}", connections);

    let mut tri_connections = HashSet::new();

    for (host, neighboors) in connections.iter() {
        // println!("=> {}", host);
        for n in neighboors {
            // look for common connections between host and neighboor = tri_connections
            let intersect = connections[n]
                .intersection(&connections[host])
                .collect_vec();
            // println!(" with {}: {:?}", n, intersect);
            for i in intersect {
                let mut tri_connection = Vec::new();
                tri_connection.push(host);
                tri_connection.push(n);
                tri_connection.push(i);
                tri_connection.sort();
                tri_connections.insert(tri_connection);
            }
        }
    }
    // filter to keep only connections with (at least) one computer begining with 't'
    tri_connections = tri_connections
        .into_iter()
        .filter(|c| c.iter().filter(|&&&el| starts_with_t(el)).count() != 0)
        .collect();
    // println!("{:?}", tri_connections);
    println!(
        "Number of trios with at least one begining with 't': {}",
        tri_connections.len()
    );
}

fn intersect_sorted_slices(s1: &[u16], s2: &[u16]) -> Vec<u16> {
    let mut inter = Vec::new();
    let mut c1 = 0;
    let mut c2 = 0;

    while c1 < s1.len() && c2 < s2.len() {
        if s1[c1] < s2[c2] {
            c1 += 1;
        } else if s1[c1] > s2[c2] {
            c2 += 1;
        } else {
            inter.push(s1[c1]);
            c1 += 1;
            c2 += 1;
        }
    }

    inter
}

#[memoize(Ignore: map)]
fn find_largest_network(map: &HashMap<u16, Vec<u16>>, candidates: Vec<u16>) -> Vec<u16> {
    let mut network = Vec::new();
    for &c in candidates.iter() {
        let intersect = intersect_sorted_slices(&map[&c], &candidates);
        // let intersect = map[&c]
        //     .intersection(&candidates_set)
        //     .cloned()
        //     .sorted()
        //     .collect_vec();
        if intersect.len() + 1 > network.len() {
            let mut subnet = find_largest_network(map, intersect);
            subnet.push(c);
            if subnet.len() > network.len() {
                network = subnet;
            }
        }
    }
    network
}

fn solve_part_2(input: &str) {
    let connections = parse_input_v2(input);
    // println!("{:?}", connections);

    let now = Instant::now();
    let largest_lan =
        find_largest_network(&connections, connections.keys().cloned().sorted().collect());

    println!(
        "LAN Password: {:?} (took {}ms)",
        largest_lan
            .iter()
            .map(|&value| computer_u16_to_name(value))
            .sorted()
            .join(","),
        now.elapsed().as_millis()
    );
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
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
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
