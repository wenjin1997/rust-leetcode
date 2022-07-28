/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

// @lc code=start
use std::cmp::min;
impl Solution {
    /// 动态规划
    /// dp[i] 表示凑够面值 i 所需的最小的硬币数量
    /// 方程
    /// dp[i] = min{dp[i - coins[j]]} + 1; // j = 0..coins.len()
    /// 
    /// 结果： dp[amount]
    /// 时间复杂度: O(n * amount) n 为 coins.len()
    /// 空间复杂度: O(amount)
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;

        for i in 1..=amount as usize {
            for coin in &coins {
                // 保证索引合法
                if i as i32 - coin >= 0 {
                    dp[i] = min(dp[i], dp[i - *coin as usize] + 1);
                    // println!("dp[{}] = {}", i, dp[i]);
                }
            }
        }

        // 表示没有满足的银币组合
        if dp[amount as usize] > amount {
            return -1;
        }

        dp[amount as usize]
    }
}
// @lc code=end

