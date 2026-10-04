pub fn part1() {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();

    let num_valid = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| is_valid(l.trim()))
        .sum::<u32>();

    println!("{}", num_valid);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();

    let num_valid = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| is_valid_anagram(l.trim()))
        .sum::<u32>();

    println!("{}", num_valid);
}

fn is_valid(input: &str) -> u32 {
    let words = input.split_whitespace().collect::<Vec<_>>();

    for i in 0..words.len() {
        for j in i + 1..words.len() {
            if words[i] == words[j] {
                return 0;
            }
        }
    }

    1
}

fn is_valid_anagram(input: &str) -> u32 {
    let words = input.split_whitespace().collect::<Vec<_>>();

    for i in 0..words.len() {
        for j in i + 1..words.len() {
            let mut chars_1 = words[i].chars().collect::<Vec<_>>();
            chars_1.sort();

            let mut chars_2 = words[j].chars().collect::<Vec<_>>();
            chars_2.sort();

            if chars_1 == chars_2 {
                return 0;
            }
        }
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, is_valid("aa bb cc dd ee"));
        assert_eq!(0, is_valid("aa bb cc dd aa"));
        assert_eq!(1, is_valid("aa bb cc dd aaa"));
    }

    #[test]
    fn test2() {
        assert_eq!(1, is_valid_anagram("abcde fghij"));
        assert_eq!(0, is_valid_anagram("abcde xyz ecdab"));
        assert_eq!(1, is_valid_anagram("a ab abc abd abf abj"));
        assert_eq!(1, is_valid_anagram("iiii oiii ooii oooi oooo"));
        assert_eq!(0, is_valid_anagram("oiii ioii iioi iiio"));
    }
}
