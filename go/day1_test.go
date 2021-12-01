package aoc_2021

import "testing"

func TestDay1Part1(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/1.txt")
	input, _ := LinesToInts(lines)
	got := Day1Part1(input)
	expected := 1154
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}

func TestDay1Part2(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/1.txt")
	input, _ := LinesToInts(lines)
	got := Day1Part2(input)
	expected := 1127
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}
