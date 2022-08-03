/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    /// 双指针解法
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        // 先对数组排序
        nums.sort();
        let len = nums.len();
        for i in 0..len {
            if nums[i] > 0 { return result; }
            // 第一个元素去重
            if i > 0 && nums[i] == nums[i - 1] { continue; }
            // 开始找第二个数和第三个数，让它们的和为 - nums[i]
            let (mut left, mut right) = (i + 1, len - 1);
            while left < right {
                if nums[i] + nums[left] + nums[right] > 0 {
                    right -= 1;
                    // 去重
                    while left < right && nums[right] == nums[right + 1] { right -= 1}
                } else if nums[i] + nums[left] + nums[right] < 0 {
                    left += 1;
                    // 去重
                    while left < right && nums[left] == nums[left - 1] { left += 1; }
                } else {
                    // 放入结果中
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    // 左右指针移动，继续寻找
                    right -= 1;
                    left += 1;
                    // 去重
                    while left < right && nums[right] == nums[right + 1] { right -= 1}
                    while left < right && nums[left] == nums[left - 1] { left += 1; }
                }
            }
        }
        result
    }
}
// @lc code=end

