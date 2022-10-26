/*
 * @lc app=leetcode id=1094 lang=java
 *
 * [1094] Car Pooling
 */

// @lc code=start
class Solution {
    public boolean carPooling(int[][] trips, int capacity) {
        int[] diff = new int[10001]; // 差分数组
        for (int[] trip : trips) {
            int passengers = trip[0]; // 要上车的乘客数
            int from = trip[1]; // 上车的地点
            int to = trip[2]; // 下车的地点
            diff[from] += passengers;
            diff[to] -= passengers;
        }

        // 还原数组
        int this_capacity = diff[0];
        if (this_capacity > capacity) return false;

        for (int i = 1; i < 10001; i++) {
            this_capacity += diff[i];
            if (this_capacity > capacity) return false;
        }

        return true;
    }
}
// @lc code=end

