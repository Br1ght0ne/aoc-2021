use std::{io::BufRead, str::FromStr};

#[derive(Debug, Default)]
pub struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Debug)]
pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (direction, distance) = (parts.next().unwrap(), parts.next().unwrap());
        let distance = distance.parse::<i32>().unwrap();
        match direction {
            "forward" => Ok(Self::Forward(distance)),
            "up" => Ok(Self::Up(distance)),
            "down" => Ok(Self::Down(distance)),
            _ => Err("invalid direction"),
        }
    }
}

pub fn process_input(input: impl BufRead) -> impl Iterator<Item = Command> {
    input
        .lines()
        .flat_map(|line| line.ok().and_then(|line| line.parse::<Command>().ok()))
}

pub fn part1(commands: impl Iterator<Item = Command>) -> i32 {
    let sub = commands.fold(Submarine::default(), |mut sub, command| {
        match command {
            Command::Forward(distance) => sub.horizontal += distance,
            Command::Up(distance) => sub.depth -= distance,
            Command::Down(distance) => sub.depth += distance,
        };
        sub
    });
    sub.horizontal * sub.depth
}

pub fn part2(commands: impl Iterator<Item = Command>) -> i32 {
    let sub = commands.fold(Submarine::default(), |mut sub, command| {
        match command {
            Command::Forward(distance) => {
                sub.horizontal += distance;
                sub.depth += sub.aim * distance;
            }
            Command::Up(distance) => sub.aim -= distance,
            Command::Down(distance) => sub.aim += distance,
        };
        sub
    });
    sub.horizontal * sub.depth
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use Command::*;

    const INPUT: &str = include_str!("../../inputs/2.txt");

    fn example_input() -> impl Iterator<Item = Command> {
        vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)].into_iter()
    }

    fn input() -> impl Iterator<Item = Command> {
        process_input(BufReader::new(INPUT.as_bytes()))
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(example_input()), 150);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(example_input()), 900);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1_635_930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1_781_819_478);
    }
}
