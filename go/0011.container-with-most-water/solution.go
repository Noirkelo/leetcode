// Created by Noirkelo at 2025/08/05 13:29
// leetgo: dev
// https://leetcode.cn/problems/container-with-most-water/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func maxArea(height []int) (ans int) {
	l := 0
	r := len(height) - 1
	for l < r {
		if height[l] < height[r] {
			current_area := height[l] * (r - l)
			ans = max(current_area, ans)
			l++
		} else {
			current_area := height[r] * (r - l)
			ans = max(current_area, ans)
			r--
		}
	}
	return
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	height := Deserialize[[]int](ReadLine(stdin))
	ans := maxArea(height)

	fmt.Println("\noutput:", Serialize(ans))
}
