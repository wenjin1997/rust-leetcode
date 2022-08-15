/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
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
impl Solution {
    /// 层序遍历
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut stack = Vec::new();
        if root.is_none() {
            return ans;
        }
        stack.push(root.unwrap());
        while !stack.is_empty() {
            let num = stack.len();
            let mut level = Vec::new();
            for _ in 0..num {
                let tmp = stack.remove(0);
                level.push(tmp.borrow_mut().val);
                if tmp.borrow_mut().left.is_some() {
                    stack.push(tmp.borrow_mut().left.take().unwrap());
                }
                if tmp.borrow_mut().right.is_some() {
                    stack.push(tmp.borrow_mut().right.take().unwrap());
                }
            }
            ans.push(level);
        }
        ans
    }
}
// @lc code=end

