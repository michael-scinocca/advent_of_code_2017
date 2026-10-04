pub fn part1() {
    let input = std::fs::read_to_string("data/day1.txt").unwrap();

    println!("{}", get_sum(input.trim()));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day1.txt").unwrap();

    println!("{}", get_sum_2(input.trim()));
}

fn get_sum(input: &str) -> u32 {
    let mut sum = 0;

    let input_bytes = input.as_bytes();

    for window in input_bytes.windows(2) {
        if window[0] == window[1] {
            sum += (window[0] as char).to_string().parse::<u32>().unwrap();
        }
    }

    if input_bytes[0] == input_bytes[input_bytes.len() - 1] {
        sum += (input_bytes[0] as char).to_string().parse::<u32>().unwrap();
    }

    sum
}

fn get_sum_2(input: &str) -> u32 {
    let mut sum = 0;

    let input_bytes = input.as_bytes();
    let length = input_bytes.len();

    for i in 0..input_bytes.len() {
        let mut next_index = i + length / 2;
        if next_index >= length {
            next_index -= length;
        }

        if input_bytes[i] == input_bytes[next_index] {
            sum += (input_bytes[i] as char).to_string().parse::<u32>().unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, get_sum("1122"));
        assert_eq!(4, get_sum("1111"));
        assert_eq!(0, get_sum("1234"));
        assert_eq!(9, get_sum("91212129"));
    }

    #[test]
    fn test2() {
        assert_eq!(6, get_sum_2("1212"));
        assert_eq!(0, get_sum_2("1221"));
        assert_eq!(12, get_sum_2("123123"));
        assert_eq!(4, get_sum_2("12131415"));
    }
}
