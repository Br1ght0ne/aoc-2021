use std::io::BufRead;

use itertools::Itertools;

pub fn process_input(input: impl BufRead) -> impl Iterator<Item = u32> {
    input
        .lines()
        .flat_map(|line| line.ok().and_then(|line| line.parse::<u32>().ok()))
}

pub fn part1(input: impl Iterator<Item = u32>) -> usize {
    input.tuple_windows().filter(|(a, b)| a < b).count()
}

pub fn part2(input: impl Iterator<Item = u32>) -> usize {
    let sums = input.tuple_windows().map(|(a, b, c)| a + b + c);
    part1(sums)
}

pub fn part2_updated(input: impl Iterator<Item = u32>) -> usize {
    input.tuple_windows().filter(|(a, _, _, d)| a < d).count()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const INPUT: &str = include_str!("../../inputs/1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(process_input(BufReader::new(INPUT.as_bytes()))), 1154);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(process_input(BufReader::new(INPUT.as_bytes()))), 1127);
    }

    #[test]
    fn test_part2_updated() {
        let input = process_input(BufReader::new(INPUT.as_bytes()));
        assert_eq!(part2_updated(input), 1127);
    }
}
