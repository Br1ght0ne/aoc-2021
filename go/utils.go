package aoc_2021

import (
	"bufio"
	"os"
	"strconv"
)

func ReadInputLines(path string) (lines []string, err error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, nil
}

func LinesToInts(lines []string) ([]int, error) {
	ints := make([]int, len(lines))
	for i, line := range lines {
		n, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
		ints[i] = n
	}
	return ints, nil
}
