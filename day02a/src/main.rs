use std::{convert::Infallible, str::FromStr};

enum Command {
    Forward(u8),
    Down(u8),
    Up(u8),
}

impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("f") {
            let value = s.strip_prefix("forward ").unwrap().parse().unwrap();
            return Ok(Command::Forward(value));
        }
        if s.starts_with("d") {
            let value = s.strip_prefix("down ").unwrap().parse().unwrap();
            return Ok(Command::Down(value));
        }
        if s.starts_with("u") {
            let value = s.strip_prefix("up ").unwrap().parse().unwrap();
            return Ok(Command::Up(value));
        }
        unreachable!()
    }
}

#[derive(Default)]
struct Coordinates {
    depth: u32,
    horizonal: u32,
}

impl Coordinates {
    fn modify(&mut self, command: &Command) {
        match command {
            Command::Forward(value) => self.horizonal += *value as u32,
            Command::Down(value) => self.depth += *value as u32,
            Command::Up(value) => self.depth -= *value as u32,
        }
    }

    fn product(&self) -> u32 {
        self.depth * self.horizonal
    }
}

fn main() {
    let mut c = Coordinates::default();
    let input = std::fs::read_to_string(r"C:\Users\USER\Documents\github\aoc2021\day02a\input.txt")
        .unwrap();
    for line in input.lines() {
        let command = line.parse::<Command>().unwrap();
        c.modify(&command);
    }
    println!("{} {}", c.depth, c.horizonal);
    println!("{}", c.product());
}
