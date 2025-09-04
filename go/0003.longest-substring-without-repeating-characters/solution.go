// Created by Noirkelo at 2025/08/06 10:30
// leetgo: dev
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func lengthOfLongestSubstring(s string) (ans int) {
	c := map[rune]int{}
	l := -1
	for r, char := range s {
		if v, ok := c[char]; ok {
			l = max(v, l)
		}
		c[char] = r
		ans = max(ans, r-l)
	}
	return
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	s := Deserialize[string](ReadLine(stdin))
	ans := lengthOfLongestSubstring(s)

	fmt.Println("\noutput:", Serialize(ans))
}
