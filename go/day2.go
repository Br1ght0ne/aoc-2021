package aoc_2021

import (
	"errors"
	"strconv"
	"strings"
)

type Submarine struct {
	horizontal, depth, aim int
}

type Command struct {
	direction Direction
	distance  int
}

type Direction int

const (
	Forward Direction = iota
	Up                = iota
	Down              = iota
)

func parseCommand(line string) (Command, error) {
	parts := strings.Split(line, " ")
	var direction Direction
	switch parts[0] {
	case "forward":
		direction = Forward
	case "up":
		direction = Up
	case "down":
		direction = Down
	default:
		return Command{}, errors.New("invalid direction")
	}
	distance, err := strconv.Atoi(parts[1])
	if err != nil {
		return Command{}, err
	}
	return Command{direction, distance}, nil
}

func ProcessInput(lines []string) ([]Command, error) {
	commands := make([]Command, len(lines))
	for i, line := range lines {
		command, err := parseCommand(line)
		if err != nil {
			return nil, err
		}
		commands[i] = command
	}
	return commands, nil
}

func Day2Part1(commands []Command) int {
	sub := Submarine{}
	for _, command := range commands {
		switch command.direction {
		case Forward:
			sub.horizontal += command.distance
		case Up:
			sub.depth -= command.distance
		case Down:
			sub.depth += command.distance
		}
	}
	return sub.horizontal * sub.depth
}

func Day2Part2(commands []Command) int {
	sub := Submarine{}
	for _, command := range commands {
		switch command.direction {
		case Forward:
			sub.horizontal += command.distance
			sub.depth += sub.aim * command.distance
		case Up:
			sub.aim -= command.distance
		case Down:
			sub.aim += command.distance
		}
	}
	return sub.horizontal * sub.depth
}
