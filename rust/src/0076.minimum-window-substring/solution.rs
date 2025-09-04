// Created by Noirkelo at 2025/09/02 15:03
// leetgo: dev
// https://leetcode.cn/problems/minimum-window-substring/

use std::{cmp::min, i32};

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len()<t.len(){
			"".into()
		}else{
			let s: Vec<u8>=s.into_bytes();
			let mut t_count: [i32; 128]=[0;128];
			let mut diff=0;
			for char in t.into_bytes(){
				t_count[char as usize]+=1
			}
			for i in t_count{
				if i>0{
					diff+=1;
				}
			}
			let mut l=0;
			let mut a_l=0;
			let mut a_r=s.len();

			for (r,&char) in s.iter().enumerate(){
				//
				 t_count[char as usize]-=1;
				if  t_count[char as usize]==0{
					diff-=1;
				}
				while  diff==0 {
					if r - l < a_r - a_l { // 找到更短的子串
						a_l = l; // 记录此时的左右端点
						a_r = r;
					}
 					let char = s[l] as usize; 
					if t_count[char] == 0 {
						// x 移出窗口之前，检查出现次数，
						// 如果窗口内 x 的出现次数和 t 一样，
						// 那么 x 移出窗口后，窗口内 x 的出现次数比 t 的少
						diff += 1;
					}
					t_count[char] += 1;
					l+=1
				}
			}

			if a_r < s.len() {
				std::str::from_utf8(&s[a_l..=a_r]).unwrap().to_owned()

			} else {
				String::new()
			}


		}

    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let t: String = deserialize(&read_line()?)?;
	let ans: String = Solution::min_window(s, t).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
