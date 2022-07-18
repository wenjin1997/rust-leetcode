/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    // dynamic programming
    pub fn climb_stairs(n: i32) -> i32 {
        // 递推公式：f(n) = f(n - 1) + f(n - 2)
        //         f(1) = 1
        //         f(2) = 2
        if n == 1 { return 1; }
        if n == 2 { return 2; }
        let mut climb_n_1 = 2; // f(n - 1)
        let mut climb_n_2 = 1; // f(n - 2)
        for i in 3..=n {
            let cur = climb_n_1 + climb_n_2;
            climb_n_2 = climb_n_1;
            climb_n_1 = cur;
            // println!("i = {}, n - 1 = {}, n - 2 = {}, cur = {}", i, climb_n_1, climb_n_2, cur);
        }
        climb_n_1
    }
}
// @lc code=end

