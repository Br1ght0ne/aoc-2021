package aoc_2021

func Day1Part1(input []int) (count int) {
	for i := 0; i+1 < len(input); i++ {
		if input[i] < input[i+1] {
			count++
		}
	}
	return count
}

func Day1Part2(input []int) int {
	sums := make([]int, len(input)-2)
	for i := 0; i+2 < len(input); i++ {
		sums[i] = input[i] + input[i+1] + input[i+2]
	}
	return Day1Part1(sums)
}
