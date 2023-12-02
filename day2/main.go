// I said I was doing every puzzle in Rust but I need to
// play catchup today so doing this one in Go.
//
// I promise I would never code like this in production!!!

package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type CubeColour string

const (
	Blue  CubeColour = "blue"
	Red   CubeColour = "red"
	Green CubeColour = "green"
)

func main() {
	if err := partOne("./puzzle1.txt"); err != nil {
		log.Fatal(err)
	}
	if err := partTwo("./puzzle1.txt"); err != nil {
		log.Fatal(err)
	}
}

func partOne(path string) error {
	lines, err := readLines(path)
	if err != nil {
		return err
	}

	games := []Game{}
	for _, line := range lines {
		game, err := GameFromLine(line)
		if err != nil {
			return err
		}
		games = append(games, game)
	}

	ids := []int{}
	redCount := 12
	greenCount := 13
	blueCount := 14
	for _, game := range games {
		if !game.Sets.HasImpossibleSet(redCount, greenCount, blueCount) {
			ids = append(ids, game.ID)
		}
	}

	sum := 0
	for _, id := range ids {
		sum += id
	}
	fmt.Println(sum)

	return nil
}

func partTwo(path string) error {
	lines, err := readLines(path)
	if err != nil {
		return err
	}

	games := []Game{}
	for _, line := range lines {
		game, err := GameFromLine(line)
		if err != nil {
			return err
		}
		games = append(games, game)
	}

	powers := []int{}
	for _, game := range games {
		powers = append(powers, game.Sets.GetMinCubePower())
	}

	sum := 0
	for _, power := range powers {
		sum += power
	}
	fmt.Println(sum)

	return nil
}

type CubeSet struct {
	Counts map[CubeColour]int
}

func (c CubeSet) CountOf(colour CubeColour) int {
	return c.Counts[colour]
}

type CubeSets []CubeSet

func (cs CubeSets) HasImpossibleSet(redCount, greenCount, blueCount int) bool {
	for _, c := range cs {
		if c.CountOf(Red) > redCount ||
			c.CountOf(Green) > greenCount ||
			c.CountOf(Blue) > blueCount {
			return true
		}
	}
	return false
}

func (cs CubeSets) GetMinCubePower() int {
	redMax := 0
	greenMax := 0
	blueMax := 0

	for _, c := range cs {
		redCount := c.CountOf(Red)
		if redCount > redMax {
			redMax = redCount
		}

		greenCount := c.CountOf(Green)
		if greenCount > greenMax {
			greenMax = greenCount
		}

		blueCount := c.CountOf(Blue)
		if blueCount > blueMax {
			blueMax = blueCount
		}
	}

	return redMax * greenMax * blueMax
}

type Game struct {
	ID   int
	Sets CubeSets
}

func GameFromLine(line string) (Game, error) {
	var game Game
	spl := strings.Split(line, ":")
	gameID, err := strconv.Atoi(strings.Split(spl[0], " ")[1])
	if err != nil {
		return game, err
	}
	game.ID = gameID

	cubeLine := strings.TrimLeft(spl[1], " ")
	cubeSets, err := CubeSetsFromLine(cubeLine)
	if err != nil {
		return game, err
	}
	game.Sets = cubeSets

	return game, nil
}

func CubeSetsFromLine(line string) ([]CubeSet, error) {
	cubesets := []CubeSet{}
	cubesetLines := strings.Split(line, ";")
	for _, line := range cubesetLines {
		cubeset := CubeSet{Counts: make(map[CubeColour]int)}
		cubes := strings.Split(line, ",")
		for _, cube := range cubes {
			spl := strings.Split(strings.TrimLeft(cube, " "), " ")
			var colour CubeColour = CubeColour(spl[1])
			count, err := strconv.Atoi(spl[0])
			if err != nil {
				return cubesets, err
			}
			cubeset.Counts[colour] = count
		}
		cubesets = append(cubesets, cubeset)
	}
	return cubesets, nil
}
