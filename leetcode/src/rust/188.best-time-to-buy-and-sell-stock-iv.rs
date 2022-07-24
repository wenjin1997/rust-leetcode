/*
 * @lc app=leetcode id=188 lang=rust
 *
 * [188] Best Time to Buy and Sell Stock IV
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    // 动态规划
    // 定义一个三维数组来记录状态：
    // dp[i][k][j] 表示第 i 天，进行 k 笔交易，j 表示是否持有股票 所获的的利润
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let n = prices.len();
        if n == 0 { return 0; }

        let mut dp = vec![vec![vec![0; 2]; k + 1]; n];
        // 初始状态
        // 第0天，无论交易多少次，不持有股票为0，持有股票利润为-prices[0]
        for kk in 0..=k {
            dp[0][kk][1] = -prices[0];
            dp[0][kk][0] = 0;
        }

        for i in 1..n {
            for kk in 0..=k {
                if kk == 0 { // 如果交易次数为0
                    // 前一天交易0次，持有，今天不动  前一天交易0次，不持有，今天买入  
                    dp[i][kk][1] = max(dp[i-1][kk][1], dp[i-1][kk][0] - prices[i]);
                    // 前一天交易0次，不持有，今天不动
                    dp[i][kk][0] = dp[i-1][kk][0];
                                   
                } else {
                    dp[i][kk][1] = max(dp[i-1][kk][1], dp[i-1][kk-1][0] - prices[i]); // 买入
                    dp[i][kk][0] = max(dp[i-1][kk][0], dp[i-1][kk][1] + prices[i]); // 卖出
                }
            }
        }
        let mut res = 0;
        for kk in 0..=k {
            res = max(res, dp[n-1][kk][0]);
        }

        res
    }
}
// @lc code=end

