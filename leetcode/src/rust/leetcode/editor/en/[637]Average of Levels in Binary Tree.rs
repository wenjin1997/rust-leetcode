//Given the root of a binary tree, return the average value of the nodes on 
//each level in the form of an array. Answers within 10⁻⁵ of the actual answer will 
//be accepted.
//
// 
// Example 1: 
// 
// 
//Input: root = [3,9,20,null,null,15,7]
//Output: [3.00000,14.50000,11.00000]
//Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, 
//and on level 2 is 11.
//Hence return [3, 14.5, 11].
// 
//
// Example 2: 
// 
// 
//Input: root = [3,9,20,15,7]
//Output: [3.00000,14.50000,11.00000]
// 
//
// 
// Constraints: 
//
// 
// The number of nodes in the tree is in the range [1, 10⁴]. 
// -2³¹ <= Node.val <= 2³¹ - 1 
// 
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 32
//48 👎 251


//leetcode submit region begin(Prohibit modification and deletion)
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
    ///
    /// 层序遍历的变体，代码模版还是一样的
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut ans = Vec::<f64>::new(); // 存放结果
        let mut stack = vec![]; // 栈，用于层序遍历
        // 如果 root 为空就直接返回
        let root = match root {
            Some(a) => a,
            None => return ans,
        };
        // 将 root 放入栈中
        stack.push(root);
        while !stack.is_empty() {
            let mut level = vec![]; // 存放一层的节点的值
            let len = stack.len();
            for _ in 0..len {
                let node = stack.remove(0);
                level.push(node.borrow().val as f64);
                if node.borrow_mut().left.is_some() {
                    stack.push(node.borrow_mut().left.take().unwrap());
                }
                if node.borrow_mut().right.is_some() {
                    stack.push(node.borrow_mut().right.take().unwrap());
                }
            }
            // 向结果中存放这一层节点的平均值
            // 这里用到了 Iterator Adapter，先求和再求平均数
            ans.push(level.into_iter().fold(0 as f64, |sum, val| sum + val) / (len as f64));
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
