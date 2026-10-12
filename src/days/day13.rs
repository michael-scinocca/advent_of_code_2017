use std::str::FromStr;

#[derive(Debug, Clone)]
struct Firewall {
    tick: u32,
    layers: Vec<Layer>,
}

impl FromStr for Firewall {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .trim()
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim());

        Ok(Firewall {
            tick: 0,
            layers: lines
                .map(|l| {
                    let mut parts = l.split(':');

                    Layer {
                        depth: parts.next().unwrap().trim().parse().unwrap(),
                        range: parts.next().unwrap().trim().parse().unwrap(),
                        scanner_position: 0,
                        scanner_direction: 1,
                    }
                })
                .collect(),
        })
    }
}

impl Firewall {
    fn tick(&mut self) {
        self.tick += 1;

        for layer in &mut self.layers {
            layer.tick();
        }
    }
}

#[derive(Debug, Clone)]
struct Layer {
    depth: u32,
    range: u32,
    scanner_position: u32,
    scanner_direction: i32,
}

impl Layer {
    fn tick(&mut self) {
        self.scanner_position = self
            .scanner_position
            .strict_add_signed(self.scanner_direction);

        if self.scanner_position >= self.range - 1 || self.scanner_position == 0 {
            self.scanner_direction = -self.scanner_direction;
        }
    }
}
pub fn part1() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();

    let mut firewall = Firewall::from_str(&input).unwrap();

    let mut score = 0;

    let mut position = 0;

    let max_depth = firewall.layers.iter().map(|l| l.depth).max().unwrap();

    loop {
        if let Some(layer) = firewall.layers.iter().find(|l| l.depth == position)
            && layer.scanner_position == 0
        {
            score += layer.depth * layer.range;
        }

        firewall.tick();

        position += 1;

        if position > max_depth {
            break;
        }
    }

    println!("{}", score);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();

    let firewall = Firewall::from_str(&input).unwrap();

    let max_depth = firewall.layers.iter().map(|l| l.depth).max().unwrap();

    let mut delay = 0;

    'outer: loop {
        let mut tick = delay;

        for position in 0..=max_depth {
            if let Some(layer) = firewall.layers.iter().find(|l| l.depth == position)
                && tick % (layer.range * 2 - 2) == 0
            {
                delay += 1;
                continue 'outer;
            }

            tick += 1;
        }

        break;
    }

    println!("{}", delay);
}
