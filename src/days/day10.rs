pub fn part1() {
    let mut data = (0..256).collect::<Vec<_>>();

    let actions = [
        147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70,
    ];

    process_data(&mut data, &actions);

    println!("{}", data[0] * data[1]);
}

pub fn part2() {}

fn process_data(data: &mut [u32], actions: &[usize]) {
    let mut index = 0;
    let mut skip_size = 0;

    for action in actions {
        reverse(data, index, *action);

        index += action;
        index += skip_size;

        skip_size += 1;

        if index > data.len() {
            index -= data.len();
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

        process_data(&mut data, &actions);

        assert_eq!(12, data[0] * data[1]);
    }
}
