/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 { return -1; }
        let (mut left, mut right) = (0, (nums.len() - 1) as i32);
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}
// @lc code=end

