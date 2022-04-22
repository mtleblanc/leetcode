func dominantIndex(nums []int) int {
    if len(nums) == 0 {
        return -1
    }
    index := 0
    max := nums[0]
    threshold := max
    for i, v := range nums {
        if i == 0 {
            continue
        }
        if v > max {
            threshold = max * 2
            max = v
            index = i
        } else if 2*v > threshold {
            threshold = 2*v
        }
    }
    if max >= threshold {
        return index
    }
    return -1
}
