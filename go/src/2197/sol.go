func gcd(a, b int) int {
	if a == 0 {
		return b
	}
	if b == 0 {
		return a
	}
	if a > b {
		return gcd(a%b, b)
	}
	return gcd(b%a, a)
}

func replaceNonCoprimes(nums []int) []int {
	for {
		ret := make([]int, 0, len(nums))
		for i, v := range nums {
			if (i == len(nums)-1) || (gcd(v, nums[i+1]) == 1) {
				for {
					if len(ret) == 0 || gcd(v, ret[len(ret)-1]) == 1 {
						ret = append(ret, v)
						break
					}
					w := ret[len(ret)-1]
					ret = ret[:len(ret)-1]
					v = w * v / gcd(w, v)
				}
			} else {
				nums[i+1] *= v / gcd(v, nums[i+1])
			}
		}
		fmt.Println(len(ret))
		if len(ret) == len(nums) {
			return ret
		}
		nums = ret

	}
}
