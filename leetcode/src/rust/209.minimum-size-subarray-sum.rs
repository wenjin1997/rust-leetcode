/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

// @lc code=start
use std::cmp::min;
impl Solution {
    /// 滑动窗口
    /// left = right = 0
    /// 先固定左指针，窗口扩到到窗口内元素的和大于等于 target
    /// 然后左指针向前一步，继续扩到窗口。
    /// 每一步记录窗口的大小，最后返回最小的窗口大小的值。
    pub fn min_sub_array_len1(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_len = n + 1;
        let (mut left, mut right) = (0, 0);
        let mut window_sum = nums[right];
        while right < n {
            while right + 1 < n && window_sum < target {
                right += 1;
                window_sum += nums[right];
            }
            
            if window_sum >= target {
                min_len = min(min_len, right - left + 1);
            }
            // println!("left {} right {} sum {}", left, right, window_sum);
            
            if left == n - 1 { break; }
            window_sum -= nums[left];
            left += 1;
        }
        if window_sum < target && min_len == n + 1 {
            return 0;
        }
        min_len as i32
    }

    /// 上面写的逻辑比较复杂，下面这个清楚一些。
    /// 右指针一次只移动一次。
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = i32::MAX;
        let mut left = 0;
        let mut window_sum = 0;

        for (right, val) in nums.iter().enumerate() {
            window_sum += val;
            while window_sum >= target {
                let window_len = (right - left + 1) as i32;
                if window_len < min_len {
                    min_len = window_len;
                }
                // 缩小窗口
                window_sum -= nums[left];
                left += 1;
            }
        }

        if min_len == i32::MAX {
            return 0;
        }
        min_len
    }
}
// @lc code=end

