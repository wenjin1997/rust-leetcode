/*
 * @lc app=leetcode id=714 lang=rust
 *
 * [714] Best Time to Buy and Sell Stock with Transaction Fee
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    /// 动态规划
    /// 
    /// 定义
    /// dp[i][0] 未持有股票 dp[i][1] 持有股票
    /// 
    /// 状态转移
    /// 1. 不动 或者 卖出股票
    ///    dp[i][0] = max(dp[i-1][0], dp[i-1][1] + prices[i])
    /// 2. 不动 或者 买入股票，同时付手续费
    ///    dp[i][1] = max(dp[i-1][1], dp[i-1][0] - prices[i] -fee)
    /// 
    /// 初始条件
    /// dp[0][0] = 0
    /// dp[0][1] = -prices[0] - fee
    /// 
    /// 结果
    /// dp[n-1][0]
    /// 
    /// 空间优化
    /// 只用两个变量记录：
    /// sell: dp[i][0]
    /// buy: dp[i][1]
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        // base case
        if n == 1 { return 0; }

        let (mut sell, mut buy) = (0, - prices[0] - fee);

        for i in 1..n {
            let (old_sell, old_buy) = (sell, buy);
            buy = max(old_buy, old_sell - prices[i] - fee);
            sell = max(old_sell, old_buy + prices[i]);
        }

        sell
    }
}
// @lc code=end

