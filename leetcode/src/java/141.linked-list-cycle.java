/*
 * @lc app=leetcode id=141 lang=java
 *
 * [141] Linked List Cycle
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    // 环检测
    // 记住龟兔赛跑
    public boolean hasCycle(ListNode head) {
        ListNode lower = head;
        ListNode fast = head;

        while (lower != null && fast != null && fast.next != null) {
            lower = lower.next;
            fast = fast.next.next;
            if (lower == fast) {
                return true;
            }
        }
        return false;
    }
}
// @lc code=end

