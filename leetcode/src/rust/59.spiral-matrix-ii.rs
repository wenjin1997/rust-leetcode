/*
 * @lc app=leetcode id=59 lang=rust
 *
 * [59] Spiral Matrix II
 */

// @lc code=start
impl Solution {
    /// 模拟螺旋的过程
    /// 时间复杂度: O(n * n)
    /// 空间复杂度：O(1) 除了返回的矩阵以外，只需要常数空间。
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        let (mut left_bound, mut right_bound) = (0, n - 1);
        let (mut upper_bound, mut lower_bound) = (0, n - 1);
        let mut num = 1;

        while num <= (n * n) as i32 {
            // 在顶部，从左向右添加元素
            if upper_bound <= lower_bound {
                for j in left_bound..=right_bound {
                    matrix[upper_bound][j] = num;
                    num += 1;
                }
                upper_bound += 1;
            }

            // 在最右边，从上向下添加元素
            if left_bound <= right_bound {
                for i in upper_bound..=lower_bound {
                    matrix[i][right_bound] = num;
                    num += 1;
                }
                right_bound -= 1;
            }

            // 在底部，从右向左添加元素
            if upper_bound <= lower_bound {
                for j in (left_bound..=right_bound).rev() {
                    matrix[lower_bound][j] = num;
                    num += 1;
                }
                lower_bound -= 1;
            }

            // 在最左边，从下向上添加元素
            if left_bound <= right_bound {
                for i in (upper_bound..=lower_bound).rev() {
                    matrix[i][left_bound] = num;
                    num += 1;
                }
                left_bound += 1;
            }
        }

        matrix
    }
}
// @lc code=end

