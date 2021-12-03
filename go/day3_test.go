package aoc_2021

import "testing"

func TestDay3Part1Example(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/3.example.txt")
	got := Day3Part1(lines)
	expected := 198
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}

func TestDay3Part2Example(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/3.example.txt")
	got := Day3Part2(lines)
	expected := 230
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}

func TestDay3Part1(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/3.txt")
	got := Day3Part1(lines)
	expected := 3_687_446
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}

func TestDay3Part2(t *testing.T) {
	lines, _ := ReadInputLines("../inputs/3.txt")
	got := Day3Part2(lines)
	expected := 4_406_844
	if got != expected {
		t.Fatalf("expected: %d, got: %d", expected, got)
	}
}
