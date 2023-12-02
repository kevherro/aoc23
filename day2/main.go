package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	games, err := makeGames("input.txt")
	if err != nil {
		fmt.Printf("failed to make games: %v", err)
	}

	func() {
		n := 0
		for _, g := range games {
			if g.isPossible() {
				n += g.id
			}
		}
		fmt.Println(n)
	}()

	func() {
		n := 0
		for _, g := range games {
			n += g.power()
		}
		fmt.Println(n)
	}()
}

func makeGames(fileName string) ([]game, error) {
	ls, err := getLines(fileName)
	if err != nil {
		return nil, err
	}
	var gs []game
	for i, l := range ls {
		g, err := newGame(i+1, l)
		if err != nil {
			return nil, fmt.Errorf("make games: %v", err)
		}
		gs = append(gs, g)
	}
	return gs, nil
}

func getLines(name string) ([]string, error) {
	f, err := os.Open(name)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	s := bufio.NewScanner(f)
	var ls []string
	for s.Scan() {
		ls = append(ls, s.Text())
	}
	if err := s.Err(); err != nil {
		return nil, err
	}
	return ls, nil
}

type game struct {
	id   int
	sets []set
}

type set struct {
	r int
	g int
	b int
}

func newGame(id int, line string) (game, error) {
	s, err := sets(line)
	if err != nil {
		return game{}, fmt.Errorf("parse sets: %v", err)
	}
	return game{id: id, sets: s}, nil
}

func sets(line string) ([]set, error) {
	after := strings.SplitN(strings.ReplaceAll(line, "; ", ";"), ": ", 2)
	if len(after) != 2 {
		return nil, fmt.Errorf("split line: %v", line)
	}
	parts := strings.Split(after[1], ";")
	var s []set
	for _, p := range parts {
		r, g, b := 0, 0, 0
		sl := strings.Split(p, ", ")
		for _, v := range sl {
			p := strings.Split(v, " ")
			if len(p) != 2 {
				return nil, fmt.Errorf("split set value %v", v)
			}
			ns, c := p[0], p[1]
			n, err := strconv.Atoi(ns)
			if err != nil {
				return nil, fmt.Errorf("parse int: %v", ns)
			}
			switch c {
			case "red":
				r = n
			case "green":
				g = n
			case "blue":
				b = n
			default:
				return nil, fmt.Errorf("unrecognized color: %v", c)
			}
		}
		s = append(s, set{r: r, g: g, b: b})
	}
	return s, nil
}

func (gm game) power() int {
	r, g, b := 0, 0, 0
	for _, s := range gm.sets {
		r, g, b = max(r, s.r), max(g, s.g), max(b, s.b)
	}
	return r * g * b
}

func (gm game) isPossible() bool {
	const (
		maxR = 12
		maxG = 13
		maxB = 14
	)
	p := true
	for _, s := range gm.sets {
		p = p && (s.r <= maxR && s.g <= maxG && s.b <= maxB)
	}
	return p
}
