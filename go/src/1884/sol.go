package main

import (
	"math"
)

func twoEggDrop(n int) int {
	return int(math.Ceil(+math.Sqrt(float64(2*n)+0.25) - 0.5))
}
