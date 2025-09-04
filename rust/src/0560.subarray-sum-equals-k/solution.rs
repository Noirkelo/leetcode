// Created by Noirkelo at 2025/08/07 14:20
// leetgo: dev
// https://leetcode.cn/problems/subarray-sum-equals-k/


use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        //dp[k]=dp[k-a[i]] +a[i]
		//连续所以必须带下标
		//dp[i][k] 在i处和为k的子数组数目,k太大了能不能用a[i]？
		let mut ans: i32=0;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len() + 1); // 预分配空间
		nums.iter().fold(0, |acc,x|{
			//acc+x'-acc
			*map.entry(acc).or_default()+=1;
			ans+= *map.entry((acc+x)-k).or_default();
			acc+x
		});

		return ans;

	}
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let k: i32 = deserialize(&read_line()?)?;
	let ans: i32 = Solution::subarray_sum(nums, k).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
