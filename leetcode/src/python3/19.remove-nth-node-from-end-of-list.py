#
# @lc app=leetcode id=19 lang=python3
#
# [19] Remove Nth Node From End of List
#

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    # 两个节点：
    # fast 和 slow 相距 n 个节点，当 fast 到达链表结尾时，slow 就刚好在倒数第 n 个节点上.
    # 为了方便 slow 好删除节点，刚开始 fast 和 slow 要相距 n + 1 个节点。
    # 步骤：
    # 1. 先让 fast 从初始位置走 n + 1 步
    # 2. 让 slow 和 fast 同时向前走, fast 走到链表末尾停止
    # 3. slow 指向倒数第 n + 1 个节点，进行删除操作
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        # 让 fast 比 slow 先走一步
        fast = head
        slow = dummy

        # fast 再走 n 步
        # fast 和 slow 相差 n + 1 步
        for i in range(n):
            fast = fast.next

        while fast:
            fast = fast.next
            slow = slow.next
        
        # 删除节点
        slow.next = slow.next.next
        return dummy.next
# @lc code=end

