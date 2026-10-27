use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day22.txt").unwrap();

    let (mut location_map, mut location) = process_input(&input);

    let infections = run_bursts(
        &mut location_map,
        &mut location,
        |state| match state {
            CellState::Clean => CellState::Infected,
            CellState::Infected => CellState::Clean,
            _ => CellState::Clean,
        },
        CellState::Infected,
        10000,
    );

    println!("{}", infections);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day22.txt").unwrap();

    let (mut location_map, mut location) = process_input(&input);

    let infections = run_bursts(
        &mut location_map,
        &mut location,
        |state| match state {
            CellState::Clean => CellState::Weakened,
            CellState::Weakened => CellState::Infected,
            CellState::Infected => CellState::Flagged,
            CellState::Flagged => CellState::Clean,
        },
        CellState::Weakened,
        10000000,
    );

    println!("{}", infections);
}

struct LocationMap {
    map: HashMap<Location, CellState>,
    bounds: (i64, i64, i64, i64),
}

impl LocationMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            bounds: (0, 0, 0, 0),
        }
    }

    fn add(&mut self, location: Location, infected: CellState) {
        if location.x < self.bounds.0 {
            self.bounds.0 = location.x;
        }

        if location.x > self.bounds.1 {
            self.bounds.1 = location.x;
        }

        if location.y < self.bounds.2 {
            self.bounds.2 = location.y;
        }

        if location.y > self.bounds.3 {
            self.bounds.3 = location.y;
        }

        self.map.insert(location, infected);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum CellState {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

fn process_input(input: &str) -> (LocationMap, Location) {
    let mut location_map = LocationMap::new();

    let mut location = Location::new();

    for line in input.trim().lines().map(|l| l.trim()) {
        location.x = 0;

        for char in line.chars() {
            if char != '#' && char != '.' {
                continue;
            }

            let infected = if char == '#' {
                CellState::Infected
            } else {
                CellState::Clean
            };

            location_map.add(location.clone(), infected);

            location.x += 1;
        }

        location.y += 1;
    }

    let starting_location = Location {
        x: (location_map.bounds.0 + location_map.bounds.1) / 2,
        y: (location_map.bounds.2 + location_map.bounds.3) / 2,
    };

    (location_map, starting_location)
}

fn run_bursts<F>(
    location_map: &mut LocationMap,
    location: &mut Location,
    cell_transition: F,
    first_state: CellState,
    bursts: u32,
) -> u64
where
    F: Fn(CellState) -> CellState,
{
    let mut infections = 0;

    let mut direction = 0;

    for _ in 0..bursts {
        let infected = {
            if let Some(infected) = location_map.map.get(location) {
                infected.clone()
            } else {
                CellState::Clean
            }
        };

        direction += match infected {
            CellState::Clean => -1,
            CellState::Infected => 1,
            CellState::Weakened => 0,
            CellState::Flagged => 2,
        };

        if direction < 0 {
            direction = 3;
        }

        if direction > 3 {
            direction -= 4;
        }

        if let Some(location_infected) = location_map.map.get_mut(location) {
            *location_infected = cell_transition(infected);

            if *location_infected == CellState::Infected {
                infections += 1;
            }
        } else {
            location_map.add(location.clone(), first_state.clone());

            if first_state == CellState::Infected {
                infections += 1;
            }
        }

        let translation = match direction {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            3 => (-1, 0),
            _ => panic!("Unknown direction"),
        };

        location.x += translation.0;
        location.y += translation.1;
    }

    infections
}

fn _render_location_map(location_map: &LocationMap, location: &Location) {
    for y in location_map.bounds.2..=location_map.bounds.3 {
        for x in location_map.bounds.0..=location_map.bounds.1 {
            let infected = {
                if let Some(infected) = location_map.map.get(&Location { x, y }) {
                    infected.clone()
                } else {
                    CellState::Clean
                }
            };

            let current_location = location.x == x && location.y == y;

            print!(
                "{}{}{}",
                if current_location { '[' } else { ' ' },
                match infected {
                    CellState::Clean => '.',
                    CellState::Infected => '#',
                    CellState::Weakened => 'W',
                    CellState::Flagged => 'F',
                },
                if current_location { ']' } else { ' ' }
            )
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = std::fs::read_to_string("data/day22.txt").unwrap();

        let (mut location_map, mut location) = process_input(&input);

        let infections = run_bursts(
            &mut location_map,
            &mut location,
            |state| match state {
                CellState::Clean => CellState::Infected,
                CellState::Infected => CellState::Clean,
                _ => CellState::Clean,
            },
            CellState::Infected,
            10000,
        );

        assert_eq!(5348, infections);
    }

    #[test]
    fn test2() {
        let input = r"
            ..#
            #..
            ...
        ";

        let (mut location_map, mut location) = process_input(&input);

        let infections = run_bursts(
            &mut location_map,
            &mut location,
            |state| match state {
                CellState::Clean => CellState::Weakened,
                CellState::Weakened => CellState::Infected,
                CellState::Infected => CellState::Flagged,
                CellState::Flagged => CellState::Clean,
            },
            CellState::Weakened,
            100,
        );

        assert_eq!(26, infections);
    }
}
