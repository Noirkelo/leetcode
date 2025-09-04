// Created by Noirkelo at 2025/08/07 14:20
// leetgo: dev
// https://leetcode.cn/problems/subarray-sum-equals-k/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func subarraySum(nums []int, k int) (ans int) {
	//区间加和直接前缀和
	sum := 0
	mapp := map[int]int{}
	mapp[0] = 1
	for _, num := range nums {
		sum += num
		ans += mapp[sum-k]
		mapp[sum] += 1
	}
	return
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	k := Deserialize[int](ReadLine(stdin))
	ans := subarraySum(nums, k)

	fmt.Println("\noutput:", Serialize(ans))
}
