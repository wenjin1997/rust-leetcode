/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    /// 方法一：暴力求解，两层循环
    /// 方法二：用哈希表，保存数值和下标
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // key: nums 中的值，value: 数字的下标
        let mut hash_map = HashMap::<i32, i32>::new();

        for (index, &value) in nums.iter().enumerate() {
            if hash_map.contains_key(&(target - value)) {
                return vec![index as i32, *hash_map.get(&(target - value)).unwrap()];
            } else {
                hash_map.insert(value, index as i32);
            }
        }

        vec![]
    }
}
// @lc code=end

