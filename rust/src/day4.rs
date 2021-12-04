use std::io::BufRead;

use arrayvec::ArrayVec;

#[derive(Debug, PartialEq)]
pub struct Cell {
    pub number: u32,
    pub marked: bool,
}

impl From<u32> for Cell {
    fn from(n: u32) -> Self {
        Self {
            number: n,
            marked: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pub cells: ArrayVec<ArrayVec<Cell, 5>, 5>,
}

impl From<ArrayVec<ArrayVec<u32, 5>, 5>> for Board {
    fn from(board: ArrayVec<ArrayVec<u32, 5>, 5>) -> Self {
        Self {
            cells: board
                .into_iter()
                .map(|row| row.into_iter().map(Cell::from).collect())
                .collect(),
        }
    }
}

impl Board {
    pub fn maybe_mark(&mut self, n: u32) -> bool {
        for row in &mut self.cells {
            for cell in row {
                if cell.number == n {
                    cell.marked = true;
                    return true;
                }
            }
        }
        false
    }

    pub fn is_winning(&self) -> bool {
        let winning_row = self
            .cells
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked));
        let winning_col = (0..5)
            .map(|col| {
                self.cells
                    .iter()
                    .map(|row| &row[col])
                    .collect::<ArrayVec<_, 5>>()
            })
            .any(|col| col.iter().all(|cell| cell.marked));
        winning_row || winning_col
    }

    fn score(&self, last_called: u32) -> u32 {
        let unmarked_sum: u32 = self
            .cells
            .iter()
            .map(|row| -> u32 {
                row.iter()
                    .filter_map(|cell| (!cell.marked).then(|| cell.number))
                    .sum()
            })
            .sum();
        eprintln!(
            "score: {} * {} = {}",
            unmarked_sum,
            last_called,
            unmarked_sum * last_called
        );
        unmarked_sum * last_called
    }
}

pub fn read_board(input: &mut impl Iterator<Item = String>) -> Option<Board> {
    let numbers: ArrayVec<ArrayVec<u32, 5>, 5> = input
        .skip(1)
        .take(5)
        .map(|row| {
            row.split_whitespace()
                .flat_map(|n| n.parse::<u32>())
                .collect()
        })
        .collect();
    if !numbers.is_empty() {
        Some(Board::from(numbers))
    } else {
        None
    }
}

pub fn process_input(input: impl BufRead) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines().flatten();
    let called = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(|s| s.parse::<u32>())
        .collect();
    let mut boards = Vec::new();
    while let Some(board) = read_board(&mut lines) {
        boards.push(board);
    }
    (called, boards)
}

pub fn part1((called, mut boards): (Vec<u32>, Vec<Board>)) -> u32 {
    for call in called {
        for board in &mut boards {
            board.maybe_mark(call);
            if board.is_winning() {
                return board.score(call);
            }
        }
    }
    unreachable!("should find a winning board")
}

pub fn part2((called, mut boards): (Vec<u32>, Vec<Board>)) -> u32 {
    for call in called {
        for board in &mut boards {
            board.maybe_mark(call);
        }
        if boards.len() == 1 {
            let board = &boards[0];
            if board.is_winning() {
                return board.score(call);
            }
        }
        boards.retain(|board| !board.is_winning());
    }
    unreachable!("should have a last winning board")
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../inputs/4.example.txt");
    const INPUT: &str = include_str!("../../inputs/4.txt");

    #[test]
    fn test_process_example_input() {
        let (called, boards) = process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()));
        assert_eq!(
            called,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(
            boards,
            vec![
                Board::from(ArrayVec::from([
                    ArrayVec::from([22, 13, 17, 11, 0]),
                    ArrayVec::from([8, 2, 23, 4, 24]),
                    ArrayVec::from([21, 9, 14, 16, 7]),
                    ArrayVec::from([6, 10, 3, 18, 5]),
                    ArrayVec::from([1, 12, 20, 15, 19])
                ])),
                Board::from(ArrayVec::from([
                    ArrayVec::from([3, 15, 0, 2, 22]),
                    ArrayVec::from([9, 18, 13, 17, 5]),
                    ArrayVec::from([19, 8, 7, 25, 23]),
                    ArrayVec::from([20, 11, 10, 24, 4]),
                    ArrayVec::from([14, 21, 16, 12, 6])
                ])),
                Board::from(ArrayVec::from([
                    ArrayVec::from([14, 21, 17, 24, 4]),
                    ArrayVec::from([10, 16, 15, 9, 19]),
                    ArrayVec::from([18, 8, 23, 26, 20]),
                    ArrayVec::from([22, 11, 13, 6, 5]),
                    ArrayVec::from([2, 0, 12, 3, 7])
                ])),
            ]
        )
    }
    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            4512
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(process_input(BufReader::new(INPUT.as_bytes()))),
            87456
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(process_input(BufReader::new(EXAMPLE_INPUT.as_bytes()))),
            1924
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
