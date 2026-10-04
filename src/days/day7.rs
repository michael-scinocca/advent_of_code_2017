use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();

    let mut programs = Vec::new();

    for line in input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim())
    {
        if !line.contains("->") {
            continue;
        }

        let mut parts = line.split("->");

        let mut name_and_weight = parts.next().unwrap().split_whitespace();

        let name = name_and_weight.next().unwrap().trim();
        let weight = name_and_weight
            .next()
            .unwrap()
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .parse::<u32>()
            .unwrap();

        let children = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.trim().to_string())
            .collect::<Vec<_>>();

        programs.push(Program {
            name: name.to_string(),
            weight,
            children,
        });
    }

    'main: for program in &programs {
        for children in programs.iter().map(|p| &p.children) {
            if children.contains(&program.name) {
                continue 'main;
            }
        }

        println!("{}", program.name);
        break;
    }
}

struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();

    let mut programs = HashMap::new();

    for line in input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim())
    {
        if line.contains("->") {
            let mut parts = line.split("->");

            let mut name_and_weight = parts.next().unwrap().split_whitespace();

            let name = name_and_weight.next().unwrap().trim();
            let weight = name_and_weight
                .next()
                .unwrap()
                .trim()
                .trim_matches(|c| c == '(' || c == ')')
                .parse::<u32>()
                .unwrap();

            let children = parts
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.trim().to_string())
                .collect::<Vec<_>>();

            programs.insert(
                name.to_owned(),
                Program {
                    name: name.to_owned(),
                    weight,
                    children,
                },
            );
        } else {
            let mut name_and_weight = line.split_whitespace();

            let name = name_and_weight.next().unwrap().trim();
            let weight = name_and_weight
                .next()
                .unwrap()
                .trim()
                .trim_matches(|c| c == '(' || c == ')')
                .parse::<u32>()
                .unwrap();

            programs.insert(
                name.to_owned(),
                Program {
                    name: name.to_owned(),
                    weight,
                    children: Vec::new(),
                },
            );
        }
    }

    let mut root_program = String::new();

    'main: for program in programs.values() {
        for children in programs.values().map(|p| &p.children) {
            if children.contains(&program.name) {
                continue 'main;
            }
        }

        root_program = program.name.to_owned();

        break;
    }

    let program = programs.get(&root_program).unwrap();

    println!("{}", find_corrected_weight(program, &programs));
}

fn find_corrected_weight(program: &Program, programs: &HashMap<String, Program>) -> u32 {
    let mut corrected_weight = None;

    resolve_weight(program, programs, &mut corrected_weight);

    corrected_weight.unwrap()
}

fn resolve_weight(
    program: &Program,
    programs: &HashMap<String, Program>,
    corrected_weight: &mut Option<u32>,
) -> u32 {
    let children = program
        .children
        .iter()
        .flat_map(|c| programs.get(c))
        .collect::<Vec<_>>();

    if children.is_empty() {
        program.weight
    } else {
        let mut child_weights = children
            .iter()
            .map(|c| (*c, resolve_weight(c, programs, corrected_weight)))
            .collect::<Vec<_>>();

        child_weights.sort_by_key(|k| k.1);

        let anomaly: Option<(&Program, u32)> = {
            if child_weights.is_empty() {
                None
            } else if child_weights[0].1 != child_weights[1].1 {
                Some(child_weights[0])
            } else if child_weights[child_weights.len() - 2].1
                != child_weights[child_weights.len() - 1].1
            {
                Some(child_weights[child_weights.len() - 1])
            } else {
                None
            }
        };

        if let Some(anomaly) = anomaly {
            let target = child_weights
                .iter()
                .find(|cw| cw.0.name != anomaly.0.name)
                .unwrap()
                .1;

            let correction = target as i32 - anomaly.1 as i32;

            if corrected_weight.is_none() {
                *corrected_weight = Some((anomaly.0.weight as i32 + correction) as u32);
            }
        }

        program.weight + child_weights.iter().map(|cw| cw.1).sum::<u32>()
    }
}
