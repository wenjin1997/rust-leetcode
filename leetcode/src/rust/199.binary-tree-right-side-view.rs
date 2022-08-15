/*
 * @lc app=leetcode id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
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
    /// 层序遍历的变体
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];

        if root.is_none() {
            return ans;
        }
        stack.push(root.unwrap());
        while !stack.is_empty() {
            let len = stack.len();
            let mut level = vec![];
            for _ in 0..len {
                let node = stack.remove(0);
                level.push(node.borrow().val);
                if node.borrow_mut().left.is_some() {
                    stack.push(node.borrow_mut().left.take().unwrap());
                }
                if node.borrow_mut().right.is_some() {
                    stack.push(node.borrow_mut().right.take().unwrap());
                }
            }
            // 向结果中加入这一层的最后一个元素
            ans.push(level[len - 1]);
        }
        ans
    }
}
// @lc code=end

