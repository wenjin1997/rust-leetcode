/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    /// 双指针
    /// left 从数组左边遍历， right 从数组最右边遍历
    /// left 维持左边的元素不是 target
    /// right 维持右边的元素都是 target
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            if nums[left] == val {
                nums[left] = nums[right - 1];
                right -= 1;
            } else {
                left += 1;
            }
        }
        left as i32
    }
}
// @lc code=end

