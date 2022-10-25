/*
 * @lc app=leetcode id=283 lang=java
 *
 * [283] Move Zeroes
 */

// @lc code=start
class Solution {
    public void moveZeroes(int[] nums) {
        // 双指针
        int left = 0, right = 0;
        while (right < nums.length) {
            if (nums[right] != 0) {
                nums[left] = nums[right];
                left++;
            }
            right++;
        }
        // 将 left 后面的都置为 0
        for (int i = left; i < nums.length; i++) {
            nums[i] = 0;
        }
    }
}
// @lc code=end

