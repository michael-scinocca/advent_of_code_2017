enum Transform {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

pub fn part1() {
    let mut data: Vec<char> = "abcdefghijklmnop".chars().collect();

    let input = std::fs::read_to_string("data/day16.txt").unwrap();

    let transforms = process_input(&input);

    for transform in transforms {
        match transform {
            Transform::Spin(length) => spin(&mut data, length),
            Transform::Exchange(index1, index2) => exchange(&mut data, index1, index2),
            Transform::Partner(program1, program2) => partner(&mut data, program1, program2),
        }
    }

    println!("{}", String::from_iter(data));
}

pub fn part2() {
    let mut data: Vec<char> = "abcdefghijklmnop".chars().collect();

    let input = std::fs::read_to_string("data/day16.txt").unwrap();

    let transforms = process_input(&input);

    let mut transform_count = 0;

    while transform_count < 1_000_000_000 {
        for transform in &transforms {
            match transform {
                Transform::Spin(length) => spin(&mut data, *length),
                Transform::Exchange(index1, index2) => exchange(&mut data, *index1, *index2),
                Transform::Partner(program1, program2) => partner(&mut data, *program1, *program2),
            }
        }

        transform_count += 1;

        if &String::from_iter(&data) == "abcdefghijklmnop" {
            transform_count = 1_000_000_000 - 1_000_000_000 % transform_count;
        }
    }

    println!("{}", String::from_iter(data));
}

fn process_input(input: &str) -> Vec<Transform> {
    let mut transforms = Vec::new();

    let parts = input.trim().split(',');

    for part in parts {
        if let Some(spin_length) = part.strip_prefix('s') {
            transforms.push(Transform::Spin(spin_length.parse().unwrap()));
        }

        if let Some(exchange) = part.strip_prefix('x') {
            let mut exchange = exchange.split('/');

            transforms.push(Transform::Exchange(
                exchange.next().unwrap().parse().unwrap(),
                exchange.next().unwrap().parse().unwrap(),
            ));
        }

        if let Some(partner) = part.strip_prefix('p') {
            let mut partner = partner.split('/');

            transforms.push(Transform::Partner(
                partner.next().unwrap().parse().unwrap(),
                partner.next().unwrap().parse().unwrap(),
            ));
        }
    }

    transforms
}

fn spin(data: &mut Vec<char>, length: usize) {
    let index = data.len() - length;

    *data = [&data[index..], &data[0..index]].concat();
}

fn exchange(data: &mut [char], index1: usize, index2: usize) {
    data.swap(index1, index2);
}

fn partner(data: &mut [char], program1: char, program2: char) {
    let index1 = data.iter().position(|c| *c == program1).unwrap();
    let index2 = data.iter().position(|c| *c == program2).unwrap();

    exchange(data, index1, index2);
}
