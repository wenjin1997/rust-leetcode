/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
    /// 前序遍历
    /// 
    /// 递归方法
    pub fn preorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 确定终止条件
        let root = match(root) {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        res.push(root.borrow().val);
        // Vec 的 append 方法：
        // 将其他的元素全部移动进来，并将之前的都置空
        //
        // RefCell 的 borrow 方法：
        // 不可变借用包装的值。
        //
        // Rc 的 clone 方法：
        // 增加 strong reference count。
        res.append(&mut Self::preorder_traversal(root.borrow().left.clone()));
        res.append(&mut Self::preorder_traversal(root.borrow().right.clone()));
        res
    }

    /// 迭代方法
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            res.push(node.borrow().val);
            if let Some(ref r) = node.borrow().right {
                stack.push(r.clone());
            }
            if let Some(ref l) = node.borrow().left {
                stack.push(l.clone());
            }
        }
        res
    }
}
// @lc code=end

