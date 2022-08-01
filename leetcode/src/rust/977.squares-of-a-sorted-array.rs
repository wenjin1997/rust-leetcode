/*
 * @lc app=leetcode id=977 lang=rust
 *
 * [977] Squares of a Sorted Array
 */

// @lc code=start
impl Solution {
    /// 双指针
    /// [-4, -1, 0, 3, 10]
    ///       ^  ^
    ///     left right
    /// left 指针指向负数， right 指针指向 0 或正数
    /// 然后比较绝对值的大小，开始移动指针
    pub fn sorted_squares1(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut right: i32 = 0;
        while right < n as i32 && nums[right as usize] < 0 {
            right += 1;
        }

        let mut ans = vec![0;n];
        let mut left = right - 1;
        for i in 0..n {
            // 处理右边指针越界情况
            if right == n as i32 {
                ans[i] = nums[left as usize] * nums[left as usize];
                left -= 1;
                continue;
            }

            // 处理左边指针越界情况
            if left == -1 {
                ans[i] = nums[right as usize] * nums[right as usize];
                right += 1;
                continue;
            }

            if nums[right as usize] <= - nums[left as usize] {
                ans[i] = nums[right as usize] * nums[right as usize];
                right += 1;
            } else {
                ans[i] = nums[left as usize] * nums[left as usize];
                left -= 1;
            }

        }

        ans
    }
    /// 双指针法
    /// 这次左右指针初始值不同
    /// left = 0, right = n - 1
    /// 哪一个的绝对值大，就将相应的平方数放到结果数组的后面。
    /// 相比于方法一，不用处理指针越界的情况。
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut left, mut right) = (0, n as i32 - 1);
        let mut ans = vec![0;n];
        let mut pos = n - 1; // 放入结果数组的指针

        while left <= right {
            if i32::abs(nums[left as usize]) > i32::abs(nums[right as usize]) {
                ans[pos] = nums[left as usize] * nums[left as usize];
                pos -= 1;
                left += 1;
            } else {
                ans[pos] = nums[right as usize] * nums[right as usize];
                pos -= 1;
                right -= 1;
            }
        }
        ans
    }
}
// @lc code=end

