/*
 * @lc app=leetcode id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 */

// @lc code=start

impl Solution {
    // 1. 优先队列方法
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len(); // 数组的长度
        let k = k as usize;
        let mut ans = vec![0; n - k + 1];
        let mut queue = std::collections::BinaryHeap::new(); // 优先队列

        // 首先，将数组中前 k 个元素装入优先队列中
        for i in 0..k {
            queue.push((nums[i], i));
        }

        // 结果的第一个元素
        if let Some(t) = queue.peek() {
            ans[0] = t.0;
        }

        // 开始滑动窗口
        for i in k..n {
            queue.push((nums[i], i));
            
            while let Some(t) = queue.peek() {
                if t.1 <= i - k {
                    queue.pop();
                } else {
                    break;
                }
            }

            if let Some(t) = queue.peek() {
                ans[i - k + 1] = t.0;
            }
        }

        ans
    }

    // 2. 单调队列
    pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        
    }
}
// @lc code=end

