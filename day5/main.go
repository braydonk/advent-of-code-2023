package main

import (
	"errors"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func main() {
	if err := partOne("./puzzle.txt"); err != nil {
		log.Fatal(err)
	}

	if err := partTwo("./puzzle_test.txt"); err != nil {
		log.Fatal(err)
	}
}

func partOne(filename string) error {
	lines, err := readLines(filename)
	if err != nil {
		return err
	}

	seedLine := lines[0]
	numberLine := strings.Split(strings.TrimLeft(strings.Split(seedLine, ":")[1], " "), " ") //lol
	seeds := []int{}
	for _, numStr := range numberLine {
		number, err := strconv.Atoi(numStr)
		if err != nil {
			return err
		}
		seeds = append(seeds, number)
	}

	lines = lines[2:]

	rangeGroups := []RangeGroup{}
	rangeLines := []string{}
	for _, line := range lines {
		if line == "" {
			newRanges, err := RangesFromLines(rangeLines)
			if err != nil {
				return err
			}
			rangeGroups = append(rangeGroups, newRanges)
			rangeLines = []string{}
		} else {
			rangeLines = append(rangeLines, line)
		}
	}

	closest := -1
	for _, seed := range seeds {
		location := seed
		for _, rangeGroup := range rangeGroups {
			location = rangeGroup.MapEachRange(location)
		}
		if closest == -1 || location < closest {
			closest = location
		}
	}

	fmt.Println(closest)

	return nil
}

func partTwo(filename string) error {
	lines, err := readLines(filename)
	if err != nil {
		return err
	}

	seedGroups, err := SeedGroupsFromLine(lines[0])
	if err != nil {
		return err
	}

	lines = lines[2:]

	rangeGroups := []RangeGroup{}
	rangeLines := []string{}
	for _, line := range lines {
		if line == "" {
			newRanges, err := RangesFromLines(rangeLines)
			if err != nil {
				return err
			}
			rangeGroups = append(rangeGroups, newRanges)
			rangeLines = []string{}
		} else {
			rangeLines = append(rangeLines, line)
		}
	}

	locationChan := make(chan int)
	for _, seedGroup := range seedGroups {
		seedGroup := seedGroup
		go func() {
			closest := -1
			closestSeed := 0
			for i := 0; i < seedGroup.Count; i++ {
				seedStart := seedGroup.Start + i
				location := seedStart
				for _, rangeGroup := range rangeGroups {
					location = rangeGroup.MapEachRange(location)
				}
				if closest == -1 || location < closest {
					closest = location
					closestSeed = seedStart
				}
			}
			fmt.Printf("got closest location %d for seed %d in group %d\n", closest, closestSeed, seedGroup.Start)
			locationChan <- closest
		}()
	}

	closest := -1
	for i := 0; i < len(seedGroups); i++ {
		location := <-locationChan
		if closest == -1 || location < closest {
			closest = location
		}
	}

	fmt.Println(closest)

	return nil
}

type Range struct {
	SrcStart  int
	SrcEnd    int
	DestStart int
	DestEnd   int
}

func RangeFromLine(line string) (Range, error) {
	nums := strings.Split(line, " ")
	srcStart, err := strconv.Atoi(nums[1])
	if err != nil {
		return Range{}, nil
	}
	destStart, err := strconv.Atoi(nums[0])
	if err != nil {
		return Range{}, nil
	}
	rangeLen, err := strconv.Atoi(nums[2])
	if err != nil {
		return Range{}, nil
	}
	return Range{
		SrcStart:  srcStart,
		SrcEnd:    srcStart + rangeLen,
		DestStart: destStart,
		DestEnd:   destStart + rangeLen,
	}, nil
}

var ErrNoRangeMatch = errors.New("no range match for this number")

func (r Range) Map(x int) (int, error) {
	if r.SrcStart <= x && x <= r.SrcEnd {
		mapTo := r.DestStart + (x - r.SrcStart)
		return mapTo, nil
	}
	return x, ErrNoRangeMatch
}

type RangeGroup []Range

func RangesFromLines(lines []string) (RangeGroup, error) {
	ranges := RangeGroup{}
	for _, line := range lines {
		newRange, err := RangeFromLine(line)
		if err != nil {
			return nil, err
		}
		ranges = append(ranges, newRange)
	}
	return ranges, nil
}

func (rs RangeGroup) MapEachRange(x int) int {
	for _, r := range rs {
		newX, err := r.Map(x)
		if err != nil {
			continue
		}
		return newX
	}
	return x
}

type SeedGroup struct {
	Start int
	Count int
}

func SeedGroupsFromLine(line string) ([]SeedGroup, error) {
	seedGroups := []SeedGroup{}
	numberLine := strings.Split(strings.TrimLeft(strings.Split(line, ":")[1], " "), " ") //lol
	for i := 0; i < len(numberLine); i += 2 {
		seedStart, err := strconv.Atoi(numberLine[i])
		if err != nil {
			return nil, err
		}
		seedCount, err := strconv.Atoi(numberLine[i+1])
		if err != nil {
			return nil, err
		}
		seedGroups = append(seedGroups, SeedGroup{
			Start: seedStart,
			Count: seedCount,
		})
	}
	return seedGroups, nil
}
