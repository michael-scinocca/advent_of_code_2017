pub fn part1() {
    let mut data = (0..256).collect::<Vec<_>>();

    let actions = [
        147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70,
    ];

    process_data(&mut data, &actions, 1);

    println!("{}", data[0] * data[1]);
}

pub fn part2() {
    let mut data = (0..256).collect::<Vec<_>>();

    let hash = get_hash(
        &mut data,
        "147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70",
    );

    println!("{}", hash);
}

fn process_data(data: &mut [u32], actions: &[u8], rounds: u32) {
    let mut index = 0_usize;
    let mut skip_size = 0;

    for _ in 0..rounds {
        for action in actions {
            reverse(data, index, *action as usize);

            index += *action as usize;
            index += skip_size;

            skip_size += 1;

            while index > data.len() {
                index -= data.len();
            }
        }
    }
}

fn reverse(data: &mut [u32], start: usize, length: usize) {
    for (count, index) in (start..start + length / 2).enumerate() {
        let pos_1 = {
            if index < data.len() {
                index
            } else {
                index - data.len()
            }
        };

        let pos_2 = {
            let pos = start + length - 1 - count;
            if pos < data.len() {
                pos
            } else {
                pos - data.len()
            }
        };

        data.swap(pos_1, pos_2);
    }
}

fn get_hash(data: &mut [u32], input: &str) -> String {
    let actions = [input.as_bytes(), &[17, 31, 73, 47, 23]].concat();

    process_data(data, &actions, 64);

    data.chunks(16)
        .map(|chunk| {
            let mut val = 0_u8;
            for c in chunk {
                val ^= *c as u8;
            }

            format!("{:02x}", val)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, 2, &[1, 0])]
    #[case(0, 3, &[2, 1, 0])]
    #[case(254, 2, &[255, 254])]
    #[case(254, 4, &[1, 0, 255, 254])]
    fn test1(#[case] start: usize, #[case] length: usize, #[case] expected: &[u32]) {
        let mut data = (0..256).collect::<Vec<_>>();

        reverse(&mut data, start, length);

        let slice = {
            if start + length - 1 < data.len() {
                &data[start..start + length]
            } else {
                &[
                    &data[start..data.len()],
                    &data[0..length - (data.len() - start)],
                ]
                .concat()
            }
        };

        assert_eq!(expected, slice);
    }

    #[test]
    fn test2() {
        let mut data = (0..5).collect::<Vec<_>>();

        let actions = [3, 4, 1, 5];

        process_data(&mut data, &actions, 1);

        assert_eq!(12, data[0] * data[1]);
    }

    #[rstest]
    #[case("", "a2582a3a0e66e6e86e3812dcb672a272")]
    #[case("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd")]
    #[case("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d")]
    #[case("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e")]
    fn test3(#[case] input: &str, #[case] expected: &str) {
        let mut data = (0..256).collect::<Vec<_>>();

        let hash = get_hash(&mut data, input);

        assert_eq!(expected, hash);
    }
}
