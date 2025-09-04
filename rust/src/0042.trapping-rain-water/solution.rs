// Created by Noirkelo at 2025/08/05 16:22
// leetgo: dev
// https://leetcode.cn/problems/trapping-rain-water/



use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::{cmp::{max, min}, collections::VecDeque};

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        //接的雨水可以分解成每个格子的水 竖着分割的结果
		//也可以一起横着分割变成横着一条一条叠起来 单调队列
		//最终结果从底部向上分解,这种事分割出来的解本身有可加性
		//每个格子的水是由左侧 最大值和右侧最大值的最小值决定
		//最终求累计值result=result+a[i]
		//假设现在在i我只需要维护
		//扫两次一次维护

		//解空间中确定解的单调性
		if height.len()<3{
			return 0;
		}
		let mut l=0;
		let mut r=height.len()-1;
		let mut lmax=0;
		let mut rmax=0;
		let mut ans=0;
		while l<r{
			lmax = max(lmax, height[l]);
            rmax = max(rmax, height[r]);
			if height[l]<height[r]{
				//lmax l  r  rmax
				//lmax l rmax
				ans+=min(lmax,rmax)-height[l] ;
				l+=1;
			}else{
				ans+=min(lmax,rmax)-height[r];
				r-=1;
			}
		}
		return ans;
		//动态规划 
		// let mut lmax: Vec<i32>=Vec::from_iter(height.clone());
		// let mut rmax: Vec<i32>=Vec::from_iter(height.clone());
		// //可以使用arraywindow
		// for i in 1..lmax.len(){
		// 	lmax[i]=max(lmax[i],lmax[i-1]);
		// }
		// for i in (0..lmax.len()-1).rev(){
		// 	rmax[i]=max(rmax[i],rmax[i+1]);
		// }

		// lmax.iter().zip(rmax).enumerate().map(|(i,(l,r))|{min(*l,r)-height[i]}).sum()
	}
}

// @lc code=end

fn main() -> Result<()> {
	let height: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::trap(height).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
