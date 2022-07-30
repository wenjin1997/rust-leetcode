/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    // 迭代法
    pub fn swap_pairs1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode { val: 0, next: head});
        let mut temp = &mut dummy_head;
        
        loop {
            let mut node1 = if temp.next.is_none() {
                break;
            } else {
                temp.next.take().unwrap()
            };
            let mut node2 = if node1.next.is_none() {
                temp.next = Some(node1);
                break;
            } else {
                node1.next.unwrap()
            };

            // 初始链表
            // temp -> node1 -> node2 -> node3 -> ...
            // 反转为：
            // temp -> node2 -> node1 -> node3 -> ...
            // 更新
            // temp = node1
            node1.next = node2.next.take();     // node1 -> node3
            node2.next = Some(node1);           // node2 -> node1
            temp.next = Some(node2);            // temp -> node2
            // as_mut 将 &mut Option<T> 转换为 Option<&mut T>
            temp = temp.next.as_mut().unwrap(); // temp = node2
            temp = temp.next.as_mut().unwrap(); // temp = node1
        }
        dummy_head.next
    }

    // 递归法
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // pub fn and_then<U, F>(self, f: F) -> Option<U>
        // where
        //     F: FnOnce(T) -> Option<U>, 
        // and_then 函数： 如果 Option 是 None 就返回 None，否则返回 f 的结果
        head.and_then( |mut n| {
            match n.next {
                // m = n.next
                Some(mut m) => {
                    n.next = Solution::swap_pairs(m.next);
                    m.next = Some(n);
                    Some(m)
                },
                None => Some(n)
            }
        })
    }
}
// @lc code=end

