/*
 * @lc app=leetcode id=72 lang=rust
 *
 * [72] Edit Distance
 */

// @lc code=start
use std::cmp::min;
impl Solution {
    /// 动态规划
    /// 1. 状态定义
    ///    dp[i][j] 表示 wrod1[0..i] => word2[0..j] 所需要的最少步数
    /// 2. 转移方程
    ///    dp[i][j] = if word1[i] == word2[j]: 
    ///                     dp[i-1][j-1] 
    ///               else 
    ///                     // Insert, Delete, Replace
    ///                     Min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1
    /// 结果：dp[m][n]
    /// 时间复杂度：O(m*n)
    /// 空间复杂度：O(m*n)
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let mut dp = vec![vec![0;n + 1];m + 1];

        // 初始化
        for i in 0..=m {
            dp[i][0] = i as i32; // 删除操作
        }
        for j in 0..=n {
            dp[0][j] = j as i32; // 插入操作
        }

        for i in 1..=m {
            for j in 1..=n {
                if word1[i-1] == word2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    // 三个数求最小再加一
                    dp[i][j] = min(min(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1]) + 1;
                }
            }
        }

        dp[m][n]
    }
}
// @lc code=end

