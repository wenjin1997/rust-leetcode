/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
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
    /// 中序遍历
    /// 
    /// 递归法
    pub fn inorder_traversal_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };
        let mut res = vec![];
        res.append(&mut Self::inorder_traversal(root.borrow().left.clone()));
        res.push(root.borrow().val);
        res.append(&mut Self::inorder_traversal(root.borrow().right.clone()));
        res
    }

    /// 迭代法
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut node = root;
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        while !stack.is_empty() || node.is_some() {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(Some(n));
            }

            if let Some(Some(n)) = stack.pop() {
                res.push(n.borrow().val);
                node = n.borrow_mut().right.take();
            }
        }
        res
    }
}
// @lc code=end

