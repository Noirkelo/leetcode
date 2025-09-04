// Created by Noirkelo at 2025/08/05 14:18
// leetgo: dev
// https://leetcode.cn/problems/3sum/

package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func threeSum(nums []int) (ans [][]int) {
	//a_i + a_j =-a_k
	//i  | j | k
	//i  | 忽略 | j | k
	//
	//假设i j 固定 找 a_k
	// 1 1 -2
	len_nums := len(nums)
	sort.Ints(nums)
	for i := 0; i < len_nums; i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		for j, k := i+1, len_nums-1; j < len_nums; j++ {
			if j > i+1 && nums[j] == nums[j-1] {
				continue
			}
			//nums[j]+nums[k]只会变得更小

			for j < k && nums[j]+nums[k]+nums[i] > 0 {
				k--
			}
			if j == k {
				break
			}
			if nums[j]+nums[k]+nums[i] == 0 {
				ans = append(ans, []int{nums[i], nums[j], nums[k]})
			}
		}
	}
	return
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	ans := threeSum(nums)

	fmt.Println("\noutput:", Serialize(ans))
}
