
func sampleStats(count []int) []float64 {
	totalSamples := 0
	sum := 0
	max := -1
	min := -1
	maxCount := 0
	var mode int

	idx := 0
	for ; count[idx] == 0; idx++ {
	}
	min = idx
	for ; idx < len(count); idx++ {
		cur := count[idx]
		sum += cur * idx
		totalSamples += cur
		if cur != 0 {
			max = idx
		}
		if cur > maxCount {
			maxCount = cur
			mode = idx
		}
	}
	ret := []float64{float64(min), float64(max), float64(sum) / float64(totalSamples), 0., float64(mode)}
	for idx = 0; idx < len(count); idx++ {
		totalSamples -= count[idx] * 2
		if totalSamples < 0 {
			if ret[3] == 0. {
				ret[3] = float64(idx)
				break
			} else {
				ret[3] += float64(idx)
				ret[3] /= 2.

				break
			}
		}
		if totalSamples == 0 && ret[3] == 0 {
			ret[3] = float64(idx)
		}
	}
	return ret

}

