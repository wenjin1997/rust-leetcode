/*
 * @lc app=leetcode id=215 lang=java
 *
 * [215] Kth Largest Element in an Array
 */

// @lc code=start
class Solution {
    public int findKthLargest(int[] nums, int k) {
        // Java 的优先队列是小根堆
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        for (int num : nums) {
            pq.offer(num);
            // 如果堆中元素大于 k 个，就删除，堆中维护的是目前为止最大的 k 个数
            if (pq.size() > k) {
                pq.poll();
            }
        }
        // 返回堆顶元素
        return pq.peek();
    }
}
// @lc code=end

