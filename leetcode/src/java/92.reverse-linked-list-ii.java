/*
 * @lc app=leetcode id=92 lang=java
 *
 * [92] Reverse Linked List II
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    // 递归方法
    public ListNode reverseBetween(ListNode head, int left, int right) {
        if (left == 1) {
            return reserveN(head, right);
        }
        head.next = reverseBetween(head.next, left - 1, right - 1);
        return head;
    }
    ListNode successor = null; // 后继节点
    // 反转 head 节点后的 n 个节点
    public ListNode reserveN(ListNode head, int n) {
        if (n == 1) {
            successor = head.next;
            return head;
        }
        ListNode last = reserveN(head.next, n - 1);
        head.next.next = head;
        head.next = successor;
        return last;
    }
}
// @lc code=end

