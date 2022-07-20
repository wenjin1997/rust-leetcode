/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
impl Solution {
    // 贪心算法
    // 每步往最远了跳，记录能跳到的最远距离
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_position: usize = 0; // 能跳到的最远距离

        for i in 0..nums.len() {
            // 表示距离最远位置之前是可到达的
            if i <= max_position {
                max_position = std::cmp::max(max_position, i + nums[i] as usize);
                // 如果已经能到达最后一个位置，直接返回 true
                if max_position >= nums.len() - 1 {
                    return true;
                }
            }
            
        }

        // 未能到达最后一个位置
        false
    }
}
// @lc code=end

