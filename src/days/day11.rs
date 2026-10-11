struct Position {
    q: i64,
    r: i64,
    s: i64,
}

impl Position {
    fn translate(&mut self, direction: &str) {
        let translation = match direction {
            "nw" => (-1, 1, 0),
            "n" => (0, 1, -1),
            "ne" => (1, 0, -1),
            "se" => (1, -1, 0),
            "s" => (0, -1, 1),
            "sw" => (-1, 0, 1),
            _ => (0, 0, 0),
        };

        self.q += translation.0;
        self.r += translation.1;
        self.s += translation.2;
    }

    fn distance(&self) -> u64 {
        ((self.q.abs() + self.r.abs() + self.s.abs()) / 2) as u64
    }
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();

    let directions = input.trim().split(',').collect::<Vec<_>>();

    let distance = run_path(&directions);

    println!("{}", distance.0);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();

    let directions = input.trim().split(',').collect::<Vec<_>>();

    let distance = run_path(&directions);

    println!("{}", distance.1);
}

fn run_path(directions: &[&str]) -> (u64, u64) {
    let mut position = Position { q: 0, r: 0, s: 0 };

    let mut max_distance = 0;

    for direction in directions {
        position.translate(direction);

        if position.distance() > max_distance {
            max_distance = position.distance();
        }
    }

    (position.distance(), max_distance)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&vec!("ne", "ne", "ne"), 3)]
    #[case(&vec!("ne", "ne", "sw", "sw"), 0)]
    #[case(&vec!("ne", "ne", "s", "s"), 2)]
    #[case(&vec!("se", "sw", "se", "sw", "sw"), 3)]
    fn test1(#[case] actions: &[&str], #[case] expected: u64) {
        assert_eq!(expected, run_path(&actions).0);
    }
}

#[allow(unused)]
mod failure {
    use std::{
        cell::RefCell,
        collections::{HashMap, HashSet, VecDeque},
        rc::Rc,
    };

    const NW: usize = 0;
    const N: usize = 1;
    const NE: usize = 2;
    const SE: usize = 3;
    const S: usize = 4;
    const SW: usize = 5;

    struct Tile {
        id: u64,
        tiles: [Option<Rc<RefCell<Tile>>>; 6],
    }

    impl Tile {
        fn new(id: u64) -> Tile {
            Tile {
                id,
                tiles: [None, None, None, None, None, None],
            }
        }

        fn direction(direction: usize) -> String {
            match direction {
                NW => "NW".to_owned(),
                N => "N".to_owned(),
                NE => "NE".to_owned(),
                SE => "SE".to_owned(),
                S => "S".to_owned(),
                SW => "SW".to_owned(),
                _ => "?".to_owned(),
            }
        }
    }

    fn run_path(actions: &[&str]) -> (Vec<String>, u64) {
        let mut id_sequence = 1;

        let mut tile = Rc::new(RefCell::new(Tile::new(id_sequence)));
        initialize_tile(&mut tile, &mut id_sequence);

        for action in actions {
            let next_tile = {
                let next_tile_option = {
                    let tile_borrow = tile.borrow_mut();

                    match *action {
                        "nw" => &mut tile_borrow.tiles[NW].clone(),
                        "n" => &mut tile_borrow.tiles[N].clone(),
                        "ne" => &mut tile_borrow.tiles[NE].clone(),
                        "se" => &mut tile_borrow.tiles[SE].clone(),
                        "s" => &mut tile_borrow.tiles[S].clone(),
                        "sw" => &mut tile_borrow.tiles[SW].clone(),
                        other => panic!("Unknown {}", other),
                    }
                };

                if let Some(next_tile) = next_tile_option {
                    initialize_tile(next_tile, &mut id_sequence);

                    Some(Rc::clone(next_tile))
                } else {
                    None
                }
            };

            if let Some(next_tile) = next_tile {
                tile = next_tile;
            }
        }

        println!("{}", tile.borrow().id);

        navigate_home(&mut tile, &mut id_sequence)
    }

    fn initialize_tile(tile: &mut Rc<RefCell<Tile>>, id_sequence: &mut u64) {
        let tile_pairs = [(NE, SW), (N, S), (NW, SE), (SW, NE), (S, N), (SE, NW)];

        let mut tile_checks = HashMap::new();
        tile_checks.insert(NE, [N, SE]);
        tile_checks.insert(N, [NW, NE]);
        tile_checks.insert(NW, [SW, N]);
        tile_checks.insert(SW, [NW, S]);
        tile_checks.insert(S, [SW, SE]);
        tile_checks.insert(SE, [NE, S]);

        for pair in tile_pairs {
            if tile.borrow().tiles[pair.0].is_some() {
                continue;
            }

            let checks = tile_checks.get(&pair.0).unwrap();

            let existing_tile = {
                if let Some(ref check) = tile.borrow().tiles[checks[0]]
                    && let Some(ref existing_tile) = check.borrow().tiles[checks[1]]
                {
                    Some(existing_tile.clone())
                } else {
                    None
                }
            };

            if let Some(existing_tile) = existing_tile {
                existing_tile.borrow_mut().tiles[pair.1] = Some(tile.clone());
                tile.borrow_mut().tiles[pair.0] = Some(existing_tile.clone());
                continue;
            }

            let existing_tile = {
                if let Some(ref check) = tile.borrow().tiles[checks[1]]
                    && let Some(ref existing_tile) = check.borrow().tiles[checks[0]]
                {
                    Some(existing_tile.clone())
                } else {
                    None
                }
            };

            if let Some(existing_tile) = existing_tile {
                existing_tile.borrow_mut().tiles[pair.1] = Some(tile.clone());
                tile.borrow_mut().tiles[pair.0] = Some(existing_tile.clone());
                continue;
            }

            *id_sequence += 1;
            let n = Rc::new(RefCell::new(Tile::new(*id_sequence)));
            n.borrow_mut().tiles[pair.1] = Some(tile.clone());
            tile.borrow_mut().tiles[pair.0] = Some(n);
        }
    }

    fn navigate_home(tile: &mut Rc<RefCell<Tile>>, id_sequence: &mut u64) -> (Vec<String>, u64) {
        if tile.borrow().id == 1 {
            return (vec![], 0);
        }

        let path = Vec::new();
        let mut steps = 0;

        let mut visited = HashSet::new();

        let mut next_steps = VecDeque::new();

        get_next_steps(
            &mut next_steps,
            tile,
            id_sequence,
            &mut visited,
            &path,
            &mut steps,
        );

        let (path, steps) = loop {
            if let Some((mut next_step, path, mut steps)) = next_steps.pop_front() {
                if next_step.borrow().id == 1 {
                    break (path, steps);
                }

                get_next_steps(
                    &mut next_steps,
                    &mut next_step,
                    id_sequence,
                    &mut visited,
                    &path,
                    &mut steps,
                );
            } else {
                break (vec![], 0);
            }
        };

        (path, steps)
    }

    #[allow(clippy::type_complexity)]
    fn get_next_steps(
        next_steps: &mut VecDeque<(Rc<RefCell<Tile>>, Vec<String>, u64)>,
        tile: &mut Rc<RefCell<Tile>>,
        id_sequence: &mut u64,
        visited: &mut HashSet<u64>,
        path: &[String],
        steps: &mut u64,
    ) {
        *steps += 1;

        initialize_tile(tile, id_sequence);

        let tile_borrow = tile.borrow();

        for i in 0..tile_borrow.tiles.len() {
            if let Some(ref next) = tile_borrow.tiles[i] {
                if visited.contains(&next.borrow().id) {
                    continue;
                }

                let mut path = path.to_owned();
                path.push(Tile::direction(i).to_string());
                next_steps.push_back((next.clone(), path, *steps));
                visited.insert(next.borrow().id);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use rstest::rstest;

        use super::*;

        #[rstest]
        #[case(&vec!("ne", "ne", "ne"), 3)]
        #[case(&vec!("ne", "ne", "sw", "sw"), 0)]
        #[case(&vec!("ne", "ne", "s", "s"), 2)]
        #[case(&vec!("se", "sw", "se", "sw", "sw"), 3)]
        fn test1(#[case] actions: &[&str], #[case] expected: u64) {
            assert_eq!(expected, run_path(&actions).1);
        }
    }
}
