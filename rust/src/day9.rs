use std::io::BufRead;

pub struct Heatmap(pub Vec<Vec<u8>>);

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub value: u8,
}

impl Heatmap {
    pub fn low_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for x in 0..self.0.len() {
            for y in 0..self.0[0].len() {
                let point = self.0[x][y];
                if [
                    (x.checked_sub(1), Some(y)),
                    (x.checked_add(1), Some(y)),
                    (Some(x), y.checked_sub(1)),
                    (Some(x), y.checked_add(1)),
                ]
                .iter()
                .filter_map(|(x, y)| {
                    x.and_then(|x| y.and_then(|y| self.0.get(x).and_then(|row| row.get(y))))
                })
                .all(|height| *height > point)
                {
                    points.push(Point { x, y, value: point })
                }
            }
        }
        points
    }
}

pub fn process_input(input: impl BufRead) -> Heatmap {
    Heatmap(
        input
            .lines()
            .flatten()
            .map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10).map(|c| c as u8))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn part1(input: Heatmap) -> usize {
    input
        .low_points()
        .iter()
        .map(|s| (s.value + 1) as usize)
        .sum()
}

pub fn part2(input: Heatmap) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../inputs/9.example.txt");
    const INPUT: &str = include_str!("../../inputs/9.txt");

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            15
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(process_input(BufReader::new(INPUT.as_bytes()))), 494);
    }

    #[test]
    #[ignore]
    fn test_example_part2() {
        assert_eq!(
            part2(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            61229
        );
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(
            part2(process_input(BufReader::new(INPUT.as_bytes()))),
            15561
        );
    }
}
