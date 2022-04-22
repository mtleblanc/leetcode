func longestOnes(nums []int, k int) int {
	// save memory by using earlier indices of nums to save indices of 0s
	idx := 0
	max := 0
	i := 0
	n := len(nums)
	thisMax := 0
	for i = 0; i < n; i++ {
		if nums[i] == 1 {
			continue
		}
		if idx-k-1 < 0 {
			thisMax = i
		} else {
			thisMax = i - nums[idx-k-1]
		}
		nums[idx] = i + 1
		idx++
		if thisMax > max {
			max = thisMax
		}
	}
	if idx-k-1 < 0 {
		return i
	}
	thisMax = i - nums[idx-k-1]
	if thisMax > max {
		return thisMax
	}
	return max
}
