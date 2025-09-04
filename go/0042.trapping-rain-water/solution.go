// Created by Noirkelo at 2025/08/05 18:18
// leetgo: dev
// https://leetcode.cn/problems/trapping-rain-water/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin
func trap(height []int) (ans int) {
    stack := []int{}
    for i, h := range height {
        for len(stack) > 0 && h > height[stack[len(stack)-1]] {
            top := stack[len(stack)-1]
            stack = stack[:len(stack)-1]
            if len(stack) == 0 {
                break
            }
            left := stack[len(stack)-1]
            curWidth := i - left - 1
            curHeight := min(height[left], h) - height[top]
            ans += curWidth * curHeight
        }
        stack = append(stack, i)
    }
    return
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	height := Deserialize[[]int](ReadLine(stdin))
	ans := trap(height)

	fmt.Println("\noutput:", Serialize(ans))
}
