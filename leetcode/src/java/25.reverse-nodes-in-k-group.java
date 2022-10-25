/*
 * @lc app=leetcode id=25 lang=java
 *
 * [25] Reverse Nodes in k-Group
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
    public ListNode reverseKGroup(ListNode head, int k) {
        if (head == null) return null;
        ListNode b = head;
        for (int i = 0; i < k; i++) {
            // 不足 k 个节点，直接返回，不用进行反转
            if (b == null) return head;
            b = b.next;
        }
        ListNode a = head;
        ListNode reverseHead = reverse(a, b);
        a.next = reverseKGroup(b, k);
        return reverseHead;
    }

    // 反转区间 [a, b) 之间的链表
    private ListNode reverse(ListNode a, ListNode b) {
        ListNode prev = null, cur = a;
        while (cur != b) {
            ListNode next = cur.next;
            cur.next = prev;
            prev = cur;
            cur = next;
        }
        return prev;
    }
}
// @lc code=end

