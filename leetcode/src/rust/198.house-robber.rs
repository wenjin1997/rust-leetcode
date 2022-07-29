/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    /// 动态规划
    /// 1. 定义
    /// dp[i] 经过第 i 家房屋能获得的最大钱数
    /// 2. 方程
    /// dp[i] = max(dp[i-1], dp[i-2] + nums[i]) // 这天偷或者不偷
    /// 
    /// 结果：dp[n]
    /// 时间复杂度: O(n)
    /// 空间复杂度: O(n)
    /// 
    /// 由于只和前两个房间中的金额有关，因此可以进行空间优化，到O(1)
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // first 表示前面第二家， second 表示挨着的那一家
        let (mut first, mut second) = (0, nums[0]);

        for i in 1..n {
            let old_second = second;
            second = max(second, first + nums[i]);
            first = old_second;
        }
        second
    }
}
// @lc code=end

