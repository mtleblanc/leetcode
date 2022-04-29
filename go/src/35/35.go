package leetcode

func searchInsert(nums []int, target int) int {
	max := len(nums) - 1
	if max == -1 {
		return 0
	}

	// check some edge cases so that we don't have to worry
	// about bounds in our loop
	if target <= nums[0] {
		return 0
	}

	if target > nums[max] {
		return max + 1
	}

	if target == nums[max] {
		return max
	}

	min := 0
	var cur int = (max + min) / 2
	// because of the edge case checking, we know
	// that cur < max, since all cases with only
	// 1 element would be caught
	// this will stay invariant

	for {
		if nums[cur] == target {
			return cur
		}

		if nums[cur] < target {
			// this will never be OOB because cur < max
			if nums[cur+1] > target {
				return cur + 1
			}
			min = cur + 1
		} else {
			max = cur - 1
		}
		cur = (max + min) / 2
	}

}
