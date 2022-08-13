/*
 * @lc app=leetcode id=145 lang=rust
 *
 * [145] Binary Tree Postorder Traversal
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
    /// 后序遍历
    /// 
    /// 递归法
    pub fn postorder_traversal_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        res.append(&mut Self::postorder_traversal(root.borrow().left.clone()));
        res.append(&mut Self::postorder_traversal(root.borrow().right.clone()));
        res.push(root.borrow().val);

        res
    }


    /// 迭代法
    /// 
    /// 先是 中 -> 右 -> 左，再对结果反转。
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        let mut stack = vec![root];
        // 先前序遍历，不过顺序是 中 -> 右 -> 左
        // 因此入栈顺序是先左后右
        while let Some(node) = stack.pop() {
            res.push(node.borrow().val);
            
            if let Some(ref l) = node.borrow().left {
                stack.push(l.clone());
            }

            if let Some(ref r) = node.borrow().right {
                stack.push(r.clone());
            }
        }

        // 反转结果
        res.into_iter().rev().collect()
    }
}
// @lc code=end

