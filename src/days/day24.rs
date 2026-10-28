use std::fmt::Display;

#[derive(Debug, Clone)]
struct Component {
    id: usize,
    ports: Vec<Port>,
}

impl Component {
    fn has_available_port(&self, port: u32) -> bool {
        self.ports.iter().any(|p| p.value == port && !p.plugged)
    }

    fn plug_port(&mut self, port: u32) {
        if let Some(port) = self
            .ports
            .iter_mut()
            .find(|p| p.value == port && !p.plugged)
        {
            port.plugged = true;
        }
    }

    fn unplug_port(&mut self, port: u32) {
        if let Some(port) = self.ports.iter_mut().find(|p| p.value == port && p.plugged) {
            port.plugged = false;
        }
    }

    fn plugged_port(&self) -> Option<u32> {
        self.ports.iter().find(|p| p.plugged).map(|p| p.value)
    }

    fn available_ports(&self) -> Vec<u32> {
        self.ports
            .iter()
            .filter(|p| !p.plugged)
            .map(|p| p.value)
            .collect()
    }
}

#[derive(Clone)]
struct Bridge {
    components: Vec<Component>,
}

impl Bridge {
    fn new() -> Self {
        Bridge {
            components: Vec::new(),
        }
    }

    fn score(&self) -> u32 {
        self.components
            .iter()
            .map(|c| c.ports.iter().map(|p| p.value).sum::<u32>())
            .sum()
    }
}

impl Display for Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.components.len() {
            let component = &self.components[i];

            write!(
                f,
                "{}/{}",
                component.ports[0].value, component.ports[1].value
            )?;

            if i < self.components.len() - 1 {
                write!(f, "--")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Port {
    value: u32,
    plugged: bool,
}

pub fn part1() {
    let input = std::fs::read_to_string("data/day24.txt").unwrap();

    let components = process_input(&input);

    let bridges = get_bridges(&components);

    println!("{}", bridges.iter().map(|b| b.score()).max().unwrap());
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day24.txt").unwrap();

    let components = process_input(&input);

    let bridges = get_bridges(&components);

    let longest = bridges.iter().map(|b| b.components.len()).max().unwrap();

    let longest_bridges = bridges.iter().filter(|b| b.components.len() == longest);

    println!("{}", longest_bridges.map(|b| b.score()).max().unwrap());
}

fn process_input(input: &str) -> Vec<Component> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let mut parts = l.trim().split('/');

            Component {
                id: i,
                ports: vec![
                    Port {
                        value: parts.next().unwrap().parse().unwrap(),
                        plugged: false,
                    },
                    Port {
                        value: parts.next().unwrap().parse().unwrap(),
                        plugged: false,
                    },
                ],
            }
        })
        .collect()
}

fn get_bridges(components: &[Component]) -> Vec<Bridge> {
    let mut bridges = Vec::new();

    let mut current_bridge = Bridge::new();

    find_bridges(components, &mut current_bridge, &mut bridges);

    bridges
}

fn find_bridges(components: &[Component], current_bridge: &mut Bridge, bridges: &mut Vec<Bridge>) {
    let needed_port = {
        if let Some(component) = current_bridge.components.last() {
            *component.available_ports().first().unwrap()
        } else {
            0
        }
    };

    for component in components {
        if current_bridge
            .components
            .iter()
            .any(|c| c.id == component.id)
        {
            continue;
        }

        if !component.has_available_port(needed_port) {
            continue;
        }

        let mut component = component.clone();
        component.plug_port(needed_port);

        if let Some(component) = current_bridge.components.last_mut() {
            component.plug_port(needed_port);
        }

        current_bridge.components.push(component);

        bridges.push(current_bridge.clone());

        find_bridges(components, current_bridge, bridges);

        let plugged_port = if let Some(last_component) = current_bridge.components.pop() {
            last_component.plugged_port()
        } else {
            None
        };

        if let Some(port) = plugged_port
            && let Some(component) = current_bridge.components.last_mut()
        {
            component.unplug_port(port);
        }
    }
}
