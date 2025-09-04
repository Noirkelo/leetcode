// Created by Noirkelo at 2025/08/05 13:30
// leetgo: dev
// https://leetcode.cn/problems/container-with-most-water/


use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::cmp::max;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l=0;
		let mut r=height.len()-1;
		let mut max_area=0;

		while l<r{
			if height[l]<height[r]{
				let cur_area=height[l]*(r-l)as i32;
				max_area=max(max_area,cur_area);
				l+=1;

			}else{
				let cur_area=height[r]*(r-l)as i32;
				max_area=max(max_area,cur_area);
				r-=1;
			}
			
		}
		return max_area;
    }
}

// @lc code=end

fn main() -> Result<()> {
	let height: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::max_area(height).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
