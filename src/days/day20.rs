#[derive(PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

struct Particle {
    id: u64,
    collided: bool,
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn tick(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

fn get_particles(input: &str) -> Vec<Particle> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let mut parts = l.split_whitespace().map(|p| p.trim().trim_matches(','));

            let mut p = parts
                .next()
                .unwrap()
                .trim_start_matches("p=<")
                .trim_end_matches('>')
                .split(',');
            let mut v = parts
                .next()
                .unwrap()
                .trim_start_matches("v=<")
                .trim_end_matches('>')
                .split(',');
            let mut a = parts
                .next()
                .unwrap()
                .trim_start_matches("a=<")
                .trim_end_matches('>')
                .split(',');

            Particle {
                id: i as u64,
                collided: false,
                position: Vector {
                    x: p.next().unwrap().parse().unwrap(),
                    y: p.next().unwrap().parse().unwrap(),
                    z: p.next().unwrap().parse().unwrap(),
                },
                velocity: Vector {
                    x: v.next().unwrap().parse().unwrap(),
                    y: v.next().unwrap().parse().unwrap(),
                    z: v.next().unwrap().parse().unwrap(),
                },
                acceleration: Vector {
                    x: a.next().unwrap().parse().unwrap(),
                    y: a.next().unwrap().parse().unwrap(),
                    z: a.next().unwrap().parse().unwrap(),
                },
            }
        })
        .collect()
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();

    let mut particles = get_particles(&input);

    for _ in 0..1000 {
        for particle in &mut particles {
            particle.tick();
        }
    }

    let closest = particles
        .iter()
        .map(|p| {
            (
                p.id,
                p.position.x.abs() + p.position.y.abs() + p.position.z.abs(),
            )
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    println!("{}", closest.0)
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();

    let mut particles = get_particles(&input);

    for _ in 0..1000 {
        for particle in &mut particles {
            particle.tick();
        }

        let mut particle_view = &mut particles[..];

        while let Some((first, rest)) = particle_view.split_first_mut() {
            for other in &mut *rest {
                if first.position == other.position {
                    first.collided = true;
                    other.collided = true;
                }
            }

            particle_view = rest;
        }

        particles.retain(|p| !p.collided);
    }

    println!("{}", particles.len());
}
