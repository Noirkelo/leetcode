// Created by Noirkelo at 2025/08/05 14:18
// leetgo: dev
// https://leetcode.cn/problems/3sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
		let nums_len=nums.len();
		let mut  ans= Vec::<Vec<i32>>::default();
		nums.sort();
		for i in 0..nums_len {
			if i>0 && nums[i-1]==nums[i]{
				continue
			}
			let mut k=nums.len()-1;
			for j in i+1..nums_len{
				if j>i+1 && nums[j-1]==nums[j]{
					continue
				}
				while j<k&& nums[j]+nums[k]+nums[i]>0{
					k-=1;
				}
				//j!=k 本身也不能当成答案
				if j==k{
					break;
				}
				//提取其中一个 ，利用有序性定下来k和j的遍历顺序
				if nums[j]+nums[k]+nums[i]==0{
					ans.push(vec![nums[i],nums[j],nums[k]]);
				}


			}
		}
		ans
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: Vec<Vec<i32>> = Solution::three_sum(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
