use std::str::Chars;

pub fn part1() {
    let input = std::fs::read_to_string("data/day9.txt").unwrap();

    let (score, _groups, _garbage) = process_input(&input);

    println!("{}", score);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day9.txt").unwrap();

    let (_score, _groups, garbage) = process_input(&input);

    println!("{}", garbage);
}

fn process_input(input: &str) -> (u32, u32, usize) {
    let mut score = 0;
    let mut groups = 0;
    let mut garbage = 0_usize;

    let mut stream = input.chars();

    while let Some(next) = stream.next() {
        if next == '!' {
            stream.next().unwrap();
        } else if next == '<' {
            process_garbage(&mut stream, &mut garbage);
        } else if next == '{' {
            let mut level = 1;
            let mut group_score = 0;
            process_group(
                &mut stream,
                &mut garbage,
                &mut level,
                &mut groups,
                &mut group_score,
            );
            score += group_score;
        }
    }

    (score, groups, garbage)
}

fn process_garbage(stream: &mut Chars<'_>, garbage_length: &mut usize) -> String {
    let mut garbage = String::new();

    loop {
        let mut next = stream.next().unwrap();

        if next == '!' {
            next = stream.next().unwrap();
        } else if next == '>' {
            *garbage_length += garbage.len();
            break garbage;
        } else {
            garbage.push(next);
        }
    }
}

fn process_group(
    stream: &mut Chars<'_>,
    garbage: &mut usize,
    level: &mut u32,
    groups: &mut u32,
    score: &mut u32,
) -> (u32, u32) {
    *score += *level;
    *groups += 1;

    loop {
        let mut next = stream.next().unwrap();

        if next == '!' {
            next = stream.next().unwrap();
        } else if next == '{' {
            *level += 1;
            process_group(stream, garbage, level, groups, score);
            *level -= 1;
        } else if next == '<' {
            process_garbage(stream, garbage);
        } else if next == '}' {
            break (*score, *level);
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("<>", "")]
    #[case("<random characters>", "random characters")]
    #[case("<<<<>", "<<<")]
    #[case("<{!>}>", "{}")]
    #[case("<!!>", "")]
    #[case("<!!!>>", "")]
    #[case("<{o\"i!a,<{i<a>", "{o\"i,<{i<a")]
    fn test_garbage(#[case] input: &str, #[case] expected: &str) {
        let mut stream = input.chars();
        stream.next().unwrap();

        let mut garbage = 0_usize;
        assert_eq!(
            expected.to_owned(),
            process_garbage(&mut stream, &mut garbage)
        );
    }

    #[rstest]
    #[case("{}", 1)]
    #[case("{{{}}}", 3)]
    #[case("{{},{}}", 3)]
    #[case("{{{},{},{{}}}}", 6)]
    #[case("{<{},{},{{}}>}", 1)]
    #[case("{<a>,<a>,<a>,<a>}", 1)]
    #[case("{{<a>},{<a>},{<a>},{<a>}}", 5)]
    #[case("{{<!>},{<!>},{<!>},{<a>}}", 2)]
    fn test_groups(#[case] input: &str, #[case] expected_score: u32) {
        assert_eq!(expected_score, process_input(input).1);
    }

    #[rstest]
    #[case("{}", 1)]
    #[case("{{{}}}", 6)]
    #[case("{{},{}}", 5)]
    #[case("{{{},{},{{}}}}", 16)]
    #[case("{<a>,<a>,<a>,<a>}", 1)]
    #[case("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9)]
    #[case("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9)]
    #[case("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3)]
    fn test_score(#[case] input: &str, #[case] expected_score: u32) {
        assert_eq!(expected_score, process_input(input).0);
    }
}
