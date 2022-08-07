/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut next_n = n;
        set.insert(n);
        while next_n != 1 {
            next_n = Self::next_num(next_n);
            // println!("next n: {}", next_n);
            if !set.insert(next_n) { return false; }
        }
        true
    }

    /// 计算 n 的下一个数字
    fn next_num(n: i32) -> i32 {
        let mut remain = n;
        let mut square_sum = 0;
        while remain > 0 {
            let add_num = remain % 10;
            remain = remain / 10;
            square_sum += add_num * add_num;
        }

        square_sum
    }
}
// @lc code=end

