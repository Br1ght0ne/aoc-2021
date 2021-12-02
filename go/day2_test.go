package aoc_2021

import "testing"

var exampleInput = []Command{
	{Forward, 5},
	{Down, 5},
	{Forward, 8},
	{Up, 3},
	{Down, 8},
	{Forward, 2},
}

func TestDay2Part1Example(t *testing.T) {
	got := Day2Part1(exampleInput)
	expected := 150
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}

func TestDay2Part2Example(t *testing.T) {
	got := Day2Part2(exampleInput)
	expected := 900
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}

func TestDay2Part1(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/2.txt")
	commands, _ := ProcessInput(lines)
	got := Day2Part1(commands)
	expected := 1635930
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}

func TestDay2Part2(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/2.txt")
	commands, _ := ProcessInput(lines)
	got := Day2Part2(commands)
	expected := 1781819478
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}
