/*
 * @lc app=leetcode id=309 lang=rust
 *
 * [309] Best Time to Buy and Sell Stock with Cooldown
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    /// 动态规划
    /// 
    /// 三个状态：
    /// f[i][0] 表示持有一支股票
    /// f[i][1] 表示处于冷冻期，未持有股票
    /// f[i][2] 表示未处于冷冻期，未持有股票
    /// 
    /// 动态转移
    /// 1. 前一天也持有股票，或者前一天未处于冷冻期，今天能买一支股票
    /// f[i][0] = max(f[i-1][0], f[i-1][2] - prices[i])
    /// 2. 前一天有一支股票，今天卖出，因此得处于冷冻期
    /// f[i][1] = f[i][0] + prices[i]
    /// 3. 前一天未处于冷冻期，没有股票，或者前一天是冷冻期，今天不再是冷冻期
    /// f[i][2] = max(f[i-1][2], f[i-1][1])
    /// 
    /// 初始条件
    /// f[0][0] = -prices[0]
    /// f[0][1] = 0
    /// f[0][2] = 0
    /// 
    /// 结果
    /// max(f[n-1][0], f[n-1][1], f[n-1][2])
    /// 由于 f[n-1][0] 是持有股票，肯定不能是最大，因此结果为：
    ///         max(f[n-1][1], f[n-1][2])
    /// 
    /// 优化：
    /// 由于都只和前一天的状态有关，因此可以进行空间优化
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // base case
        let n = prices.len();
        if n == 1 { return 0; }

        let (mut f0, mut f1, mut f2) = (-prices[0], 0, 0);
        for i in 1..n {
            let (prev_f0, prev_f1, prev_f2) = (f0, f1, f2);
            f0 = max(prev_f0, prev_f2 - prices[i]);
            f1 = prev_f0 + prices[i];
            f2 = max(prev_f2, prev_f1);
        }

        max(f1, f2)
    }
}
// @lc code=end

