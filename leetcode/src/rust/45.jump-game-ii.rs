/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */

// @lc code=start
impl Solution {
    // 贪婪算法
    // 局部最优得到全局最优
    // 也就是每一步尽可能跳最远
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_position = 0 as usize; // 记录能跳到的最远的距离
        let mut end = 0 as usize; // 边界，做步数更新
        let mut step = 0; // 步数
        for i in 0..(nums.len() - 1) {
            // 跳到的最远距离
            max_position = std::cmp::max(max_position, i + nums[i] as usize);
            if (i == end) {
                end = max_position;
                step += 1;
            }
        }
        step
    }
}
// @lc code=end

