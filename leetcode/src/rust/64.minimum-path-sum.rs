/*
 * @lc app=leetcode id=64 lang=rust
 *
 * [64] Minimum Path Sum
 */

// @lc code=start
use std::cmp::min;
impl Solution {
    /// 动态规划
    /// dp[i, j] 表示从 [0, 0] 走到 [i,j] 的最小路径和
    /// 动态转移方程
    /// dp[i, j] = min(dp[i - 1, j], dp[i, j - 1]) + grid[i, j]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        
        // 初始化
        dp[0][0] = grid[0][0];
        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }
        
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j];
            }
        }
        
        dp[m-1][n-1]
    }
}
// @lc code=end

