use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();

    let programs = find_programs(&input);

    println!("{}", programs);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();

    let programs = find_program_groups(&input);

    println!("{}", programs);
}

fn find_programs(input: &str) -> u32 {
    let comm_map = build_comm_map(input);

    let mut programs = vec![0_u32];

    add_programs(0, &comm_map, &mut programs);

    programs.len() as u32
}

fn find_program_groups(input: &str) -> u32 {
    let comm_map = build_comm_map(input);

    let mut program_groups: Vec<Vec<u32>> = vec![];

    'outer: for key in comm_map.keys() {
        for programs in &program_groups {
            if programs.contains(key) {
                continue 'outer;
            }
        }

        let mut programs = vec![*key];

        add_programs(*key, &comm_map, &mut programs);

        program_groups.push(programs);
    }

    program_groups.len() as u32
}

fn build_comm_map(input: &str) -> HashMap<u32, Vec<u32>> {
    let lines = input
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim());

    let mut comm_map = HashMap::new();

    for line in lines {
        let mut from_to = line.split("<->");

        let from = from_to.next().unwrap().trim().parse::<u32>().unwrap();
        let to = from_to
            .next()
            .unwrap()
            .split(',')
            .map(|t| t.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        comm_map
            .entry(from)
            .and_modify(|f: &mut Vec<_>| f.extend(to.clone()))
            .or_insert(to);
    }

    comm_map
}

fn add_programs(program: u32, comm_map: &HashMap<u32, Vec<u32>>, programs: &mut Vec<u32>) {
    for partner in &comm_map[&program] {
        if programs.contains(partner) {
            continue;
        }

        programs.push(*partner);

        add_programs(*partner, comm_map, programs);
    }
}
