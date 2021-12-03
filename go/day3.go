package aoc_2021

import "strconv"

func Day3Part1(input []string) int {
	positions := countPositions(input)
	gamma, epsilon := gammaRate(positions), epsilonRate(positions)
	return gamma * epsilon
}

type Position struct{ ones, zeroes int }

func (p *Position) mostCommon() rune {
	if p.ones >= p.zeroes {
		return '1'
	} else {
		return '0'
	}
}

func (p *Position) leastCommon() rune {
	if p.ones < p.zeroes {
		return '1'
	} else {
		return '0'
	}
}

func countPositions(input []string) []Position {
	positions := make([]Position, len(input[0]))
	for _, line := range input {
		for i, c := range line {
			switch c {
			case '0':
				positions[i].zeroes++
			case '1':
				positions[i].ones++
			}
		}
	}
	return positions
}

func gammaRate(positions []Position) int {
	gamma := make([]rune, len(positions))
	for i, position := range positions {
		gamma[i] = position.mostCommon()
	}
	n, _ := strconv.ParseInt(string(gamma), 2, 0)
	return int(n)
}

func epsilonRate(positions []Position) int {
	epsilon := make([]rune, len(positions))
	for i, position := range positions {
		epsilon[i] = position.leastCommon()
	}
	n, _ := strconv.ParseInt(string(epsilon), 2, 0)
	return int(n)
}

func Day3Part2(input []string) int {
	oxygen := oxygenGeneratorRating(input)
	co2 := co2ScrubberRating(input)
	return oxygen * co2
}

func oxygenGeneratorRating(input []string) int {
	pred := func(s string, i int, p Position) bool { return rune(s[i]) == p.mostCommon() }
	return findOneByBitCriteria(input, pred)
}

func co2ScrubberRating(input []string) int {
	pred := func(s string, i int, p Position) bool { return rune(s[i]) == p.leastCommon() }
	return findOneByBitCriteria(input, pred)
}

func findOneByBitCriteria(input []string, pred func(string, int, Position) bool) int {
	for len(input) > 1 {
		for i := 0; i < len(input[0]); i++ {
			positions := countPositions(input)
			pred := func(s string) bool { return pred(s, i, positions[i]) }
			input = retain(input, pred)
			if len(input) == 1 {
				break
			}
		}
	}
	n, _ := strconv.ParseInt(input[0], 2, 0)
	return int(n)
}

func retain(input []string, pred func(string) bool) (output []string) {
	for _, line := range input {
		if pred(line) {
			output = append(output, line)
		}
	}
	return output
}
