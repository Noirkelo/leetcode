// Created by Noirkelo at 2025/09/04 15:17
// leetgo: dev
// https://leetcode.cn/problems/binary-tree-inorder-traversal/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
enum State{
	Val(i32),
	Node(Option<Rc<RefCell<TreeNode>>>)
}
impl Solution {
    pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut ans=vec![];
		//let mut stack: Vec<_>=vec![root];
		let mut stack: Vec<_>=vec![State::Node(root)];
		//按状态去写比较简单

		// ans.extend(Solution::inorder_traversal(root.borrow().left.clone()));
		// ans.push(root.borrow().val);
		// /ans.extend(Solution::inorder_traversal(root.borrow().right.clone()));

		while let Some(cur)=stack.pop(){
			match cur{
				State::Val(val) =>{
					ans.push(val);
				},
				State::Node(Some(node)) => {
					stack.push(State::Node(node.borrow().right.clone()));
					stack.push(State::Val(node.borrow().val));
					stack.push(State::Node(node.borrow().left.clone()));
				},
				_=>{

				}
			}
		}


		ans
    }
}

// @lc code=end

fn main() -> Result<()> {
	let root: BinaryTree = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::inorder_traversal(root.into()).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
