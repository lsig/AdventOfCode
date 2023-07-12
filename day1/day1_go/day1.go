package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func maxIndex(arr []int64) int64 {
	if len(arr) == 0 {
		return 0
	}

	max := arr[0]
	maxIndex := 0

	for index, value := range arr {
		if value > max {
			max = value
			maxIndex = index
		}
	}
	return int64(maxIndex)
}

func deleteIndex(arr []int64, index int64) []int64 {
	copy(arr[index:], arr[index+1:])
	return arr[:len(arr)-1]
}

func topThreeElves(arr []int64) int64 {
	var combined_calories int64 = 0

	for i := 0; i < 3; i++ {
		index := maxIndex(arr)
		combined_calories += arr[index]
		arr = deleteIndex(arr, index)
	}
	return int64(combined_calories)
}

func main() {
	file, err := os.Open("input.txt")
	check(err)

	var total_cal int64 = 0
	var calories_arr []int64

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			calories_arr = append(calories_arr, total_cal)
			total_cal = 0
			continue
		}

		calories, parseErr := strconv.ParseInt(line, 0, 64)
		check(parseErr)

		total_cal += calories

	}

	max_calories_index := maxIndex(calories_arr)
	max_calories := calories_arr[max_calories_index]
	top_three := topThreeElves(calories_arr)

	fmt.Printf("The elve with the most calories has: %d calories", max_calories)
	fmt.Println()
	fmt.Printf("The top three elves have: %d calories", top_three)
	fmt.Println()

}
