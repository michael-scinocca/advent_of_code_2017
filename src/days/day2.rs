pub fn part1() {
    let input = std::fs::read_to_string("data/day2.txt").unwrap();

    println!("{}", get_checksum(&input));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day2.txt").unwrap();

    println!("{}", get_checksum_2(&input));
}

fn get_checksum(input: &str) -> u32 {
    let mut checksum = 0;

    let lines = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim());

    for line in lines {
        let values = line
            .split_whitespace()
            .filter(|p| !p.trim().is_empty())
            .map(|p| p.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        checksum += values.iter().max().unwrap() - values.iter().min().unwrap();
    }

    checksum
}

fn get_checksum_2(input: &str) -> u32 {
    let mut checksum = 0;

    let lines = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim());

    for line in lines {
        let values = line
            .split_whitespace()
            .filter(|p| !p.trim().is_empty())
            .map(|p| p.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        for i in 0..values.len() {
            for j in 0..values.len() {
                if i == j {
                    continue;
                }

                if values[i] % values[j] == 0 {
                    checksum += values[i] / values[j];
                    break;
                }
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
            5 1 9 5
            7 5 3
            2 4 6 8
        ";

        assert_eq!(18, get_checksum(input));
    }

    #[test]
    fn test2() {
        let input = r"
            5 9 2 8
            9 4 7 3
            3 8 6 5
        ";

        assert_eq!(9, get_checksum_2(input));
    }
}
