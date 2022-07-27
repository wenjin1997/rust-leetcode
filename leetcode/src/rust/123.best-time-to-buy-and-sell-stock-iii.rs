/*
 * @lc app=leetcode id=123 lang=rust
 *
 * [123] Best Time to Buy and Sell Stock III
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    /// 空间优化版本
    /// 也可以用三维 dp 数组，dp[i][t][j]
    /// i 表示第 i 天， t 表示股票交易次数， j 表示是否持有股票
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // base case
        let n = prices.len();
        if n == 1 { return 0; }

        // dp[i][0][0] = 0 , 因为未进行交易，未持有股票
        // dp[i][0][1] 不可能出现，因为不能出现在未进行交易的情况下持有股票

        // sell1 : dp[i][1][0] 进行一笔交易，未持有股票
        // buy1 : dp[i][1][1] 进行一笔交易，持有股票
        // 初始值：
        // dp[0][1][0] = 0, dp[0][1][1] = -prices[0]
        let (mut sell1, mut buy1) = (0, -prices[0]);

        // sell2 : dp[i][2][0] 进行两笔交易，未持有股票
        // buy2 : dp[i][2][1] 进行两笔交易，持有股票
        // 初始值：
        // dp[0][2][0] = 0, dp[0][2][1] = -prices[0]
        let (mut sell2, mut buy2) = (0, -prices[0]);

        for i in 1..n {
            // 本来应该是要先保存 old_buy 或者 old_sell 的，
            // 但是，可以转换为在同一天也能进行买和卖的转移，不影响
            // 最终结果，因此直接用 buy 和 sell 进行转移

            // dp[i][1][1] = max(dp[i-1][1][1], dp[i-1][0][0] - prices[i])
            // dp[i-1][0][0] = 0
            buy1 = max(buy1, -prices[i]);
            // dp[i][1][0] = max(dp[i-1][1][0], dp[i-1][1][1] + prices[i])
            sell1 = max(sell1, buy1 + prices[i]);
            // dp[i][2][1] = max(dp[i-1][2][1], dp[i-1][1][0] - prices[i])
            buy2 = max(buy2, sell1 - prices[i]);
            // dp[i][2][0] = max(dp[i-1][2][0], dp[i-1][2][1] + prices[i])
            sell2 = max(sell2, buy2 + prices[i]);   
        }
        // 最后结果应该是 max(0, sell1, sell2)
        // 但是 sell1 可以向 sell2 转移
        // 看作是又在当天买进再卖出，相当于利润未增加，就从 sell1 转移到 sell2 了
        sell2
    }
}
// @lc code=end

