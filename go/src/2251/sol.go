func fullBloomFlowers(flowers [][]int, persons []int) []int {
	timeChanges := make(map[int]int)
	for _, f := range flowers {
		timeChanges[f[0]] += 1
		timeChanges[f[1]+1] -= 1
	}
	times := make([]int, 0, len(timeChanges))
	for t := range timeChanges {
		times = append(times, t)
	}
	sort.Ints(times)
	vals := make([]int, len(timeChanges)+1)
	for i := range vals {
		if i == 0 {
			continue
		}
		vals[i] = vals[i-1] + timeChanges[times[i-1]]
	}
	ret := make([]int, len(persons))
	for i, p := range persons {
		idx := sort.SearchInts(times, p)
		if idx < len(times) && times[idx] == p {
			idx += 1
		}
		ret[i] = vals[idx]
	}
	return ret
}
