var dp []map[int]int



func getDp(avail int, target int, nums []int) int {
    ret, ok := dp[avail][target]
    if ok {
        return ret
    }
    if avail == 0 {
        if target == 0 { 
            return 1 
        }
        return 0
    }
    dp[avail][target] = getDp(avail - 1, target + nums[avail -1], nums) + getDp(avail - 1, target - nums[avail - 1], nums)
    return dp[avail][target]
}

func findTargetSumWays(nums []int, target int) int {
    dp = make([]map[int]int, len(nums)+1)
    for i := range dp {
        dp[i] = make(map[int]int)
    }
    return getDp(len(nums), target, nums)
}
