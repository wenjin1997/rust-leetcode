/*
 * @lc app=leetcode id=120 lang=rust
 *
 * [120] Triangle
 */

// @lc code=start
impl Solution {
    // 动态规划
    // 从底部往上推
    // 1. 定义
    //    DP[i, j] 表示从底部走到 (i, j) 位置的最小路径和
    // 2. 方程
    //    DP[i, j] = triangle(i, j) + min(DP[i + 1, j], DP[i + 1, j + 1])
    // 初始状态
    //    DP[m - 1, j] = triangle(m - 1, j)
    // 结果为 DP[0, 0]
    // 这道题由于每次都是一层一层遍历，因此可以进行状态压缩，
    // 空间上用一个一维数组。
    // 时间复杂度：O(m x n) 空间复杂度：O(1)
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let m = triangle.len();     // 行数
        if m == 1 { return triangle[0][0]; }
        
        let mut minimum = triangle[m - 1].clone();

        for i in (0..=(m - 2)).rev() {
            for j in 0..=i {
                minimum[j] = triangle[i][j] + std::cmp::min(minimum[j + 1], minimum[j]);
            }
        }
        
        minimum[0]
    }
}
// @lc code=end

