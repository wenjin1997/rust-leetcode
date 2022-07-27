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
    pub fn max_profit1(prices: Vec<i32>) -> i32 {
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

    /// 空间优化版本
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        // base case
        if n == 1 { return 0; }

        let mut buy = -prices[0]; // 买入了股票，持有股票
        let mut sell = 0;         // 卖出了股票，未持有股票

        for i in 1..n {
            let old_buy = buy;
            let old_sell = sell;
            // 动态转移方程
            buy = max(old_buy, old_sell - prices[i]);
            sell = max(old_sell, old_buy + prices[i]);
        }
        sell
    }
}
// @lc code=end

