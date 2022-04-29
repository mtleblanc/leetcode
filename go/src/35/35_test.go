package leetcode

import (
	"testing"
)

func Test(t *testing.T) {
	input := []int{1, 3, 5, 6}
	target := 7
	output := 4

	want := searchInsert(input, target)

	if want != output {
		t.Fatalf(`searchInsert = %q, want match for %#q, nil`, output, want)
	}
}
