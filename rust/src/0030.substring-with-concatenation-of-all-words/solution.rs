// Created by Noirkelo at 2025/08/07 09:34
// leetgo: dev
// https://leetcode.cn/problems/substring-with-concatenation-of-all-words/


use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::{cmp::min, collections::HashMap};

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
		let m=words[0].len();
		let n=words.len();
        if s.len()<m{
			return vec![];
		}
		let mut ans=vec![];
		let s=s.into_bytes();
		//sLen-m*n
		//s.len()-1-s.len()+m*n+1
		for offset in 0..min(m,s.len()-m*n+1){
			let mut count: HashMap<&[u8],isize>=HashMap::default();
			//
			let mut differ=0;
			for i in 0..n {
				//0
				*count.entry(&s[offset..][i*m..(i+1)*m]).or_default()+=1;
			}
			for word in words.iter(){
				//
				let v=count.entry(word.as_bytes()).or_default();
				*v-=1;
			}
			differ=count.values().filter(|v|**v!=0).count();
			if differ==0{
				ans.push(offset as i32);
			}
			//m*n
			//start 开始至少要有m*n
			//start<?'
			//sLen-1-?+1=m*n
			//?=sLen-m*n
			for st in (offset+m..s.len()-m*n+1).step_by(m){
				// if st==8{
				// 	dbg!(st);
				// 	dbg!(String::from_utf8_lossy(&s[st..][(n-1)*m..m*n]));
				// 	dbg!(String::from_utf8_lossy(&s[st-m..st]));
				// 	dbg!(&count.iter().map(|(k,v)|{(String::from_utf8_lossy(k),v)}).collect::<Vec<_>>());
				// 	dbg!(differ);

				// }
				let r=count.entry(&s[st..][(n-1)*m..m*n]).or_default();
			    if *r == -1 {
					differ-=1;
                }else if *r==0{
					differ+=1;
				}
				*r+=1;
				let l=count.entry(&s[st-m..st]).or_default();
			    if *l == 1 {
					differ-=1;
                }else if *l==0{
					differ+=1;
				}
				*l-=1;
				if differ==0{
					ans.push(st as i32);
				}
			}
			//左侧减少的和右侧增加的
			//offset +(i*m..(i+1)*m)+((n-1)*m...n*m)
		}
		ans
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let words: Vec<String> = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::find_substring(s, words).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
