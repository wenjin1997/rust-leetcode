/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 * 
 * 动态规划
 * 
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        // base case
        if n == 1 { return 0; }
        let mut dp = vec![vec![0; 2]; n]; // dp 数组

        // 初始化状态
        dp[0][0] = 0;
        dp[0][1] = -prices[0];

        for i in 1..n {
            // 动态转移方程
            dp[i][1] = max(dp[i-1][1], /* 前一天持有股票 */
                        dp[i-1][0] - prices[i]); // 前一天没有股票，今日卖出
            dp[i][0] = max(dp[i-1][0], /* 前一天也没有股票，保持不动 */
                        dp[i-1][1] + prices[i]); // 前一天持有股票，今日卖出
        }
        dp[n - 1][0]
    }
}
// @lc code=end

