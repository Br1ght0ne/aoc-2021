use std::io::BufRead;

#[derive(Debug, Default, Clone)]
pub struct Position {
    ones: usize,
    zeroes: usize,
}

impl Position {
    pub fn most_common(&self) -> char {
        if self.ones >= self.zeroes {
            '1'
        } else {
            '0'
        }
    }

    pub fn least_common(&self) -> char {
        if self.ones < self.zeroes {
            '1'
        } else {
            '0'
        }
    }
}

pub struct GammaRate(u32);
pub struct EpsilonRate(u32);

impl From<&[Position]> for GammaRate {
    fn from(positions: &[Position]) -> Self {
        let s = positions
            .iter()
            .map(|p| p.most_common())
            .collect::<String>();
        Self(u32::from_str_radix(&s, 2).unwrap())
    }
}

impl From<&[Position]> for EpsilonRate {
    fn from(positions: &[Position]) -> Self {
        let s = positions
            .iter()
            .map(|p| p.least_common())
            .collect::<String>();
        Self(u32::from_str_radix(&s, 2).unwrap())
    }
}

pub fn process_input(input: impl BufRead) -> impl Iterator<Item = String> {
    input.lines().flatten()
}

fn position_counts(input: impl Iterator<Item = String>) -> Vec<Position> {
    let mut input = input.peekable();
    let bit_count = input.peek().expect("no first number").len();
    let mut positions = vec![Position::default(); bit_count];
    for line in input {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => positions[i].zeroes += 1,
                '1' => positions[i].ones += 1,
                _ => panic!("invalid digit"),
            }
        }
    }
    positions
}

pub fn part1(input: impl Iterator<Item = String>) -> u32 {
    let positions = position_counts(input);
    let (gamma, epsilon) = (
        GammaRate::from(&positions[..]),
        EpsilonRate::from(&positions[..]),
    );
    gamma.0 * epsilon.0
}

fn retain_by_bit_criteria(
    mut numbers: Vec<String>,
    criteria: impl Fn(&String, usize, &Position) -> bool,
) -> u32 {
    for i in 0..numbers[0].len() {
        let positions = position_counts(numbers.iter().cloned());
        numbers.retain(|number| criteria(number, i, &positions[i]));
        if numbers.len() == 1 {
            break;
        }
    }
    u32::from_str_radix(&numbers[0], 2).unwrap()
}

fn oxygen_generator_rating(numbers: Vec<String>) -> u32 {
    retain_by_bit_criteria(numbers, |number, i, position| {
        number.as_bytes()[i] as char == position.most_common()
    })
}

fn co2_scrubber_rating(numbers: Vec<String>) -> u32 {
    retain_by_bit_criteria(numbers, |number, i, position| {
        number.as_bytes()[i] as char == position.least_common()
    })
}

pub fn part2(input: impl Iterator<Item = String>) -> u32 {
    let input = input.collect::<Vec<_>>();
    let oxygen = oxygen_generator_rating(input.clone());
    let co2 = co2_scrubber_rating(input);
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../inputs/3.example.txt");
    const INPUT: &str = include_str!("../../inputs/3.txt");

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            198,
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            230,
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(process_input(BufReader::new(INPUT.as_bytes()))),
            3_687_446
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(process_input(BufReader::new(INPUT.as_bytes()))),
            4_406_844
        );
    }
}
