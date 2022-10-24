/*
 * @lc app=leetcode id=2 lang=java
 *
 * [2] Add Two Numbers
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
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        int remain = 0; // 保存进位的数
        ListNode dummy = new ListNode(-1);
        ListNode p = dummy;
        while (l1 != null || l2 != null) {
            int num1 = (l1 == null) ? 0 : l1.val;
            int num2 = (l2 == null) ? 0 : l2.val;
            int sum = num1 + num2 + remain;

            int thisSum = sum % 10;
            remain = sum / 10;

            p.next = new ListNode(thisSum);
            p = p.next;
            if (l1 != null) l1 = l1.next;
            if (l2 != null) l2 = l2.next;
        }

        if (remain != 0) {
            p.next = new ListNode(remain);
        }

        return dummy.next;
    }
}
// @lc code=end

