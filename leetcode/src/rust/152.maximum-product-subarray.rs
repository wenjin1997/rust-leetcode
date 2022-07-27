/*
 * @lc app=leetcode id=152 lang=rust
 *
 * [152] Maximum Product Subarray
 */

// @lc code=start
use std::cmp::{max, min};
impl Solution {
    /// 动态规划
    /// 定义
    /// dp[i][0] 表示s[0..=i] 的子序列最大乘积 
    /// dp[i][1] 表示s[0..=i] 的子序列最小乘积
    /// 也就是 s[i] 一定选上的子序列
    /// 
    /// 转移方程
    /// dp[i][0] = a[i] >= 0 max(dp[i-1][0] * a[i], a[i])
    ///            a[i] < 0  min(dp[i-1][1] * a[i], a[i])
    /// dp[i][1] = a[i] >= 0 max(dp[i-1][1] * a[i], a[i])
    ///            a[i] < 0  min(dp[i-1][0] * a[i], a[i])
    /// 即
    /// dp[i][0] = max(a[i], dp[i-1][0] * a[i], dp[i-1][1] * a[i])
    /// dp[i][1] = min(a[i], dp[i-1][1] * a[i], dp[i-1][0] * a[i])
    /// 
    /// 结果
    /// 在 dp[0..n][0] 中的最大值
    /// 
    /// 优化
    /// 由于只用到了前一步的值，可以进行空间上的优化
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return nums[0]; }

        let (mut cur_min, mut cur_max) = (nums[0], nums[0]);
        let mut res_max = nums[0];
        for i in 1..n {
            if nums[i] >= 0 {
                cur_max = max(nums[i], cur_max * nums[i]);
                cur_min = min(nums[i], cur_min * nums[i]);
            } else {
                let old_max = cur_max;
                cur_max = max(nums[i], cur_min * nums[i]);
                cur_min = min(nums[i], old_max * nums[i]);
            }
            res_max = max(res_max, cur_max);
        }
        res_max
    }
}
// @lc code=end

