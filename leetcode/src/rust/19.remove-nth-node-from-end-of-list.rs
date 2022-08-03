/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // -1 -> 1 -> 2 -> 3 -> 4 -> 5
        // ^^^   ^^^
        // dummy head
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        // -1 -> 1 -> 2 -> 3 -> 4 -> 5
        // ^^^ 
        // right  and left
        let mut right = dummy.clone();
        let mut left = dummy.as_mut();

        // -1 -> 1 -> 2 -> 3 -> 4 -> 5
        // ^^^       ^^^ 
        // left     right
        for _ in 0..n {
            right = right.next.unwrap();
        }

        // -1 -> 1 -> 2 -> 3 -> 4 -> 5
        //                ^^^       ^^^ 
        //                left     right
        while let Some(node) = right.next {
            right = node;
            left = left.next.as_mut().unwrap();
        }
        
        // 删除倒数第二个节点
        left.next = left.next.as_mut().unwrap().next.clone();

        dummy.next
    }
}
// @lc code=end

