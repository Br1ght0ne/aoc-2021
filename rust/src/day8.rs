use std::{io::BufRead, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct Wire(pub char);

impl TryFrom<char> for Wire {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='g' => Ok(Self(value)),
            _ => Err("invalid char for wire"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Segment(pub Vec<Wire>);

impl TryFrom<&Segment> for usize {
    type Error = &'static str;

    fn try_from(Segment(wires): &Segment) -> Result<Self, Self::Error> {
        match wires.len() {
            2 => Ok(1),
            4 => Ok(4),
            3 => Ok(7),
            7 => Ok(8),
            _ => Err("invalid segment"),
        }
    }
}

impl FromStr for Segment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wires = s.chars().map(Wire::try_from).collect::<Result<_, _>>()?;
        Ok(Self(wires))
    }
}

pub struct InputLine {
    pub patterns: Vec<Segment>,
    pub display: Vec<Segment>,
}

impl InputLine {
    pub fn count_easy_in_display(&self) -> usize {
        self.display
            .iter()
            .filter_map(|s| usize::try_from(s).ok())
            .count()
    }

    pub fn sum_display(&self) -> usize {
        self.display
            .iter()
            .map(|s| usize::try_from(s).unwrap())
            .rev()
            .zip(0..)
            .map(|(n, e)| n * 10_usize.pow(e as u32))
            .sum()
    }
}

pub fn process_input(input: impl BufRead) -> Vec<InputLine> {
    input
        .lines()
        .flatten()
        .map(|line| -> InputLine {
            let mut patterns = line
                .split(&['|', ' '][..])
                .filter(|s| !s.is_empty())
                .flat_map(|segment| segment.trim().parse::<Segment>())
                .collect::<Vec<_>>();
            debug_assert_eq!(patterns.len(), 14);
            let display = patterns.split_off(10);
            InputLine { patterns, display }
        })
        .collect()
}

pub fn part1(input: Vec<InputLine>) -> usize {
    input.iter().map(|line| line.count_easy_in_display()).sum()
}

pub fn part2(input: Vec<InputLine>) -> usize {
    input.iter().map(|line| line.sum_display()).sum()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../inputs/8.example.txt");
    const INPUT: &str = include_str!("../../inputs/8.txt");

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            26
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(process_input(BufReader::new(INPUT.as_bytes()))), 362);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            61229
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(process_input(BufReader::new(INPUT.as_bytes()))),
            15561
        );
    }
}
