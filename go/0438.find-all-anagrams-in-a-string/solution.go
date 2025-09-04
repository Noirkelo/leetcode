// Created by Noirkelo at 2025/08/06 11:00
// leetgo: dev
// https://leetcode.cn/problems/find-all-anagrams-in-a-string/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func findAnagrams(s string, p string) (ans []int) {
	sLen, pLen := len(s), len(p)
	if pLen > sLen {
		return
	}
	var count [26]int
	for i, ch := range p {

		count[ch-'a']--
		count[s[i]-'a']++
	}

	differ := 0
	for _, c := range count {
		if c != 0 {
			differ++
		}
	}
	if differ == 0 {
		ans = append(ans, 0)
	}

	for i, ch := range s[:sLen-pLen] {
		if count[ch-'a'] == 1 { // 窗口中字母 s[i] 的数量与字符串 p 中的数量从不同变得相同
			differ--
		} else if count[ch-'a'] == 0 { // 窗口中字母 s[i] 的数量与字符串 p 中的数量从相同变得不同
			differ++
		}
		count[ch-'a']--

		if count[s[i+pLen]-'a'] == -1 { // 窗口中字母 s[i+pLen] 的数量与字符串 p 中的数量从不同变得相同
			differ--
		} else if count[s[i+pLen]-'a'] == 0 { // 窗口中字母 s[i+pLen] 的数量与字符串 p 中的数量从相同变得不同
			differ++
		}
		count[s[i+pLen]-'a']++

		if differ == 0 {
			ans = append(ans, i+1)
		}
	}
	//p len <= s len
	//
	return
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	s := Deserialize[string](ReadLine(stdin))
	p := Deserialize[string](ReadLine(stdin))
	ans := findAnagrams(s, p)

	fmt.Println("\noutput:", Serialize(ans))
}
