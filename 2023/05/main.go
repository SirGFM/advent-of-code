package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"os"
	"strconv"
	"strings"
	"sync"
)

type Mapping struct {
	src   int64
	dst   int64
	count int64
}

func (m Mapping) RangeFrom(src, count int64) int64 {
	if src + count < m.src || src > m.src + m.count {
		return src
	}

	delta := src - m.src
	return m.dst + delta
}

type dstMap map[string][]Mapping
type srcMap map[string]dstMap

type state func(line string) state

func min(a, b int64) int64 {
	if a < b {
		return a
	}
	return b
}

func parseMap(file io.Reader) ([]int64, []string, srcMap) {
	var curSrc string
	var curDst string

	var seeds []int64
	var srcs []string
	src2dst := srcMap{}

	var getEntries state

	getEntries = func(line string) state {
		values := strings.Split(line, " ")

		dst, err := strconv.ParseInt(values[0], 0, 64)
		if err != nil {
			panic(err)
		}
		src, err := strconv.ParseInt(values[1], 0, 64)
		if err != nil {
			panic(err)
		}
		count, err := strconv.ParseInt(values[2], 0, 64)
		if err != nil {
			panic(err)
		}

		entry := Mapping{
			dst:   dst,
			src:   src,
			count: count,
		}

		arr := src2dst[curSrc][curDst]
		src2dst[curSrc][curDst] = append(arr, entry)
		return getEntries
	}

	getMap := func(line string) state {
		name := strings.Split(line, " ")[0]
		parts := strings.Split(name, "-")
		curSrc = parts[0]
		curDst = parts[2]

		if _, ok := src2dst[curSrc]; !ok {
			src2dst[curSrc] = make(dstMap)
		}

		if len(srcs) == 0 {
			srcs = append(srcs, curSrc)
		}
		srcs = append(srcs, curDst)

		return getEntries
	}

	getWantedSeeds := func(line string) state {
		values := strings.Split(line, " ")[1:]
		for _, value := range values {
			if len(value) == 0 {
				continue
			}

			seed, err := strconv.ParseInt(value, 0, 64)
			if err != nil {
				panic(err)
			}
			seeds = append(seeds, seed)
		}

		return getMap
	}

	state := getWantedSeeds
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.Trim(line, " \n\r\t")
		if len(line) == 0 {
			state = getMap
			continue
		}

		state = state(line)
	}

	return seeds, srcs, src2dst
}

func find(src2dst srcMap, srcs []string, value int64) int64 {
	count := int64(1)

	for i := 0; i < len(srcs) - 1; i++ {
		src := srcs[i]
		dst := srcs[i+1]

		for _, entry := range src2dst[src][dst] {
			// Check for overlap.
			dist := entry.src + entry.count / 2 - (value + count / 2)
			if math.Abs(float64(dist)) > float64((entry.count + count) / 2) {
				continue
			}

			// Find the intersection.
			start := entry.src
			if value > start {
				start = value
			}
			end := min(value + 1, entry.src + entry.count)

			// Map to the next destination.
			count = end - start
			value = entry.RangeFrom(start, count)
			break
		}
	}

	return value
}

func part1(src2dst srcMap, srcs []string, seeds []int64) {
	found := int64(-1)
	for _, seed := range seeds {
		value := find(src2dst, srcs, seed)
		if found == -1 {
			found = value
		}
		found = min(found, value)
	}

	fmt.Println(found)
}

func part2(src2dst srcMap, srcs []string, seeds []int64) {
	num_threads := 100

	var wg sync.WaitGroup
	conn := make(chan int64, num_threads*2)
	res := make(chan int64, num_threads*2)

	for i := 0; i < num_threads; i++ {
		wg.Add(1)
		go func(w *sync.WaitGroup) {
			defer w.Done()

			for seed := range conn {
				value := find(src2dst, srcs, seed)
				res <- value
			}
		}(&wg)
	}

	var finished sync.WaitGroup
	finished.Add(1)
	go func(w *sync.WaitGroup){
		defer w.Done()

		found := int64(-1)
		for value := range res {
			if found == -1 {
				found = value
			}
			found = min(found, value)
		}

		fmt.Println(found)
	}(&finished)

	for i := 0; i < len(seeds); i += 2 {
		seed := seeds[i]
		count := seeds[i+1]

		for i := int64(0); i < count; i++ {
			conn <- seed + i
		}
	}

	close(conn)
	wg.Wait()
	close(res)
	finished.Wait()
}

func main() {
	seeds, srcs, src2dst := parseMap(os.Stdin)

	part1(src2dst, srcs, seeds)
	part2(src2dst, srcs, seeds)
}
