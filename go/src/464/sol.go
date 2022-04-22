package main

var dp []map[int]bool

func getDp(target int, availMask int, max int) bool {
	val, ok := dp[target][availMask]
	if ok {
		return val
	}
	canWin := false
	for i := 0; i < max; i++ {
		bit := 1 << i
		if availMask&bit != 0 {
			if i+1 >= target {
				canWin = true
			}
			canWin = canWin || !getDp(target-i-1, availMask & ^bit, max)
		}
	}
	dp[target][availMask] = canWin
	return canWin
}

func canIWin(max int, desiredTotal int) bool {
	if max*(max+1) < 2*desiredTotal {
		return false
	}
	dp = make([]map[int]bool, desiredTotal+1)
	for j := range dp {
		dp[j] = make(map[int]bool)
	}
	return getDp(desiredTotal, (1<<max)-1, max)
}
