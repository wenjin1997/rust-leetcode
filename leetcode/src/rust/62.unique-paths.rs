/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    // 动态规划
    // 小机器人只能向右或向下走，那么如果 dp[i][j] 表示
    // 到达(i, j) 时的不同路线，则它的值取决于它到它上面
    // 和左边的不同路径的值。
    // 因此状态转移方程可以这样写：
    // dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
    //
    // 初始的值，也就是在最左边和最上面，只能向下或向右，均为1。
    // dp[i][0] = 1
    // dp[0][j] = 1
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![1; n]; m];
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m - 1][n - 1]
    }
}
// @lc code=end

