// Created by Noirkelo at 2025/08/07 13:01
// leetgo: dev
// https://leetcode.cn/problems/substring-with-concatenation-of-all-words/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin

func findSubstring(s string, words []string) (ans []int) {
	//将字符串看做是'a'这题其实就是乱序里面把‘a’换成字符串
	//用map来处理，词频变成字符串频率   k的频率
	//字符串之间是乱序的也就是
	//很容易想到我需要先分组
	//.... m*n ...
	sLen, wsLen, wLen := len(s), len(words), len(words[0])
	//处理明显不可能的边界
	if wLen > sLen {
		return
	}
	//计算第一个偏移
	//保证到最后一个字符串必须是n+M
	//offset+wsLen*wLen<sLen
	//ababab
	// 把字符串分n组的不同情况有多少个
	for offset := 0; offset < wLen && offset+wLen*wsLen-1 < sLen; offset++ {
		count := map[string]int{}
		for j := 0; j < wsLen; j++ {
			count[s[offset+j*wLen:offset+(j+1)*wLen]]++
		}
		differ := 0
		for _, word := range words {
			count[word]--
		}
		for _, word := range count {
			if word != 0 {
				differ++
			}
		}
		if differ == 0 {
			ans = append(ans, offset)
		}
		//该确定分组下面我要怎么挪?
		//头已经被考虑了现在从offset+wLen开始
		//start+n*m-1<sLen
		for start := offset + wLen; start < sLen-wLen*wsLen+1; start += wLen {
			//l

			l := s[start-wLen : start]
			if count[l] == 1 {
				differ--
			} else if count[l] == 0 {
				differ++
			}
			count[l]--

			r := s[start+(wsLen-1)*wLen : start+wsLen*wLen]
			if count[r] == -1 {
				differ--
			} else if count[r] == 0 {
				differ++
			}
			count[r]++
			if differ == 0 {
				ans = append(ans, start)
			}

		}
	}
	return
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	s := Deserialize[string](ReadLine(stdin))
	words := Deserialize[[]string](ReadLine(stdin))
	ans := findSubstring(s, words)

	fmt.Println("\noutput:", Serialize(ans))
}
