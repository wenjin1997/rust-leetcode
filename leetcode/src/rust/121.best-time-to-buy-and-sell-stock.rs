/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
use std::cmp::{min, max};

impl Solution {
    // dynamic programming
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0; // 最大收益
        let mut min_price = prices[0]; // 截止到当天的历史最低价格

        for value in prices {
            max_profit = max(max_profit, value - min_price); // 更新最大收益
            min_price = min(min_price, value); // 更新历史最低价格
        }
        max_profit
    }
}
// @lc code=end

