/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 */

// @lc code=start
impl Solution {
    // 二分搜索
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        // 闭区间搜索
        while left <= right {
            let mid = left + (right - left) / 2;
            let num_mid = nums[mid as usize];
            if num_mid == target {
                return mid;
            } else if num_mid < target {
                left = mid + 1;
            } else if num_mid > target {
                right = mid - 1;
            }
        }
        -1
    }
}
// @lc code=end

