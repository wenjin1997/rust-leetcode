/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    // 动态规划
    // dp[i] 表示 nums[..=i] 最大连续子数组和，这里注意是连续的，
    //      并且包含 nums[i]，就是以nums[i]结尾的最大连续子数组和
    // 状态转移方程 1
    // dp[i] = (dp[i - 1] + nums[i] if dp[i - 1] > 0
    //          nums[i]             if dp[i - 1] <= 0)
    // 或者 状态转移方程 2
    // dp[i] = max(dp[i - 1] + nums[i], nums[i])
    // 意思是要不要下一个元素加入到前面的连续子数组中，值得就加入，
    // 否则就另起炉灶。
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut dp = 0;
        for value in nums {
            dp = max(dp + value, value);
            if dp > ans {
                ans = dp;
            }
        }
        ans
    }
}
// @lc code=end

