// Created by Noirkelo at 2025/08/06 10:01
// leetgo: dev
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/


use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::cmp::{max, min};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
		//i-1 和 i 在解的关系上
		//1.已经能分割出来动态规划
		//2.分解左边界其实是单调递增，没有更新的时候保留原始值
		//双指针这个解法其实是动态规格更进一步(做了一次空间优化dp[i] -> dp)
		//组装长度/确定左右边界
		/*
		可能存在某种化简
		max(A, B)	≡ -min(-A, -B)
		min(A, B)	≡ -max(-A, -B)
		*/
		//关注解的框框的单调性
		let mut common: [i32; 128]=[-1;128];
		let mut l: i32=-1;
		let mut ans=0;
		for (r,v) in s.into_bytes().iter().enumerate(){
			l=max(common[*v as usize],l);
			common[*v as usize]=r as i32;
			ans=max(r as i32 -l,ans);
		}
		// let mut l=0;
		// let mut ans=0;
		// //dp[i-1]+1
		// //i-common[v]
		// for (ri,v) in s.into_bytes().iter().enumerate(){
		// 	l=min(l+1,r-common[*v as usize]);
		// 	common[*v as usize]=r as i32;
		// 	ans=max(dp,ans);
		// }
		ans
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: i32 = Solution::length_of_longest_substring(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
