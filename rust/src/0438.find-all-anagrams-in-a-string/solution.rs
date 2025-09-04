// Created by Noirkelo at 2025/08/06 11:37
// leetgo: dev
// https://leetcode.cn/problems/find-all-anagrams-in-a-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let slen=s.len();
		let plen=p.len();
		if slen<plen{
			return vec![]
		}else{
			let mut ans=vec![];
			let mut count=[0;26];
			let s=s.into_bytes();
			let p=p.into_bytes();
			for (i,ch) in p.iter().enumerate(){
				count[(*ch-'a' as u8)as usize ]-=1;
				count[(s[i]-'a' as u8)as usize ]+=1;
			}
			let mut differ:i32=count.iter().filter(|x|**x!=0).count() as i32;
			if differ==0{
				ans.push(0);
			}
			for (i,ch) in s.iter().take(s.len()-p.len()).enumerate(){
				if count[(*ch-'a' as u8)as usize]==1{
					differ-=1;
				}else if count[(*ch-'a' as u8)as usize]==0{
					differ+=1;
				}
				count[(*ch-'a' as u8)as usize ]-=1;
				if count[(s[i+p.len()]-'a' as u8)as usize]==-1{
					differ-=1;
				}else if count[(s[i+p.len()]-'a' as u8)as usize]==0{
					differ+=1;
				}
				count[(s[i+p.len()]-'a' as u8)as usize ]+=1;
				if differ == 0 {
					ans.push(i as i32 + 1);
				}

			}
			return ans;
		}

	}
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let p: String = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::find_anagrams(s, p).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
