// Created by Noirkelo at 2025/08/07 15:45
// leetgo: dev
// https://leetcode.cn/problems/sliding-window-maximum/


use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        //分组维护前缀和后缀和
		let mut deque=VecDeque::<(usize,i32)>::new();
		let mut ans=Vec::<i32>::new();
		for (i,num) in nums.iter().enumerate(){
			while let Some((_,v))=deque.back(){
				if v<num{
					deque.pop_back();
				}else{
					break;
				}
			}
			deque.push_back((i,*num));
			if i==k as usize-1{
				ans.push(deque.front().unwrap().1);

			}
			if i>=k as usize{
				while let Some((idx,_))=deque.front(){
					if *idx<i-k as usize+1{
						deque.pop_front();
					}else{
						break;
					}
				}
				ans.push(deque.front().unwrap().1);

			}
			
		}

		//假设n个元素 k 窗口
		//8-3
		ans
    }
}
//aab a
//包含a的最小子串/包含b的最小子串 s[i]
//包含ab的最小子串
//
// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let k: i32 = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::max_sliding_window(nums, k).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
