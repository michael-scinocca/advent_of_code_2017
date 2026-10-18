pub fn part1() {
    let mut current_a = 873;
    let mut current_b = 583;

    let mut matches = 0;

    for _ in 0..40_000_000_usize {
        current_a = generate(current_a, 16807);
        current_b = generate(current_b, 48271);

        if current_a as u16 == current_b as u16 {
            matches += 1;
        }
    }

    println!("{}", matches);
}

pub fn part2() {
    let mut current_a = 873;
    let mut current_b = 583;

    let mut matches = 0;

    for _ in 0..5_000_000_usize {
        current_a = {
            loop {
                current_a = generate(current_a, 16807);

                if current_a % 4 == 0 {
                    break current_a;
                }
            }
        };
        current_b = {
            loop {
                current_b = generate(current_b, 48271);

                if current_b % 8 == 0 {
                    break current_b;
                }
            }
        };

        if current_a as u16 == current_b as u16 {
            matches += 1;
        }
    }

    println!("{}", matches);
}

fn generate(current: u64, factor: u64) -> u64 {
    (current * factor) % 2147483647
}
