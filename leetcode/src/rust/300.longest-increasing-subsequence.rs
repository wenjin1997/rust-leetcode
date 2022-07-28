/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    /// 动态规划
    /// dp[i] 表示选中 nums[i] 后 nums[0..=i] 的最长递增子序列长度
    /// 动态转移方程
    /// dp[i] = max(dp[j]) + 1
    ///         j = 0..i, and nums[j] < nums[i]
    /// dp[i] 初始值为 1
    /// 时间复杂度 O(n^2)
    /// 空间复杂度 O(n)
    pub fn length_of_lis1(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n]; // dp 数组
        let mut res = 1;

        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = max(dp[i], dp[j] + 1);
                }
            }
            res = max(res, dp[i]);
        }
        res
    }

    /// 二分查找
    /// 维护一个LIS数组，里面是最大长度递增子序列
    /// a. 维护数组 LIS
    /// b. for i in 0..n
    ///     二分插入到LIS中
    ///     找到比nums[i] 小的一个元素，更新它
    ///     如果nums[i] 是最大的，就追加到LIS数组后面
    /// c. 返回 LIS.size()
    /// 时间复杂度：O(nlogn)
    /// 空间复杂度：O(n)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return 1; }
        let mut lis = Vec::<i32>::new();
        lis.push(nums[0]);
        for i in 1..nums.len() {
            if nums[i] > lis[lis.len() - 1] {
                lis.push(nums[i]);
            } else {
                // 找到第一个比 nums[i] 小的下标
                // 二分查找，找左侧边界
                let (mut left, mut right) = (0, (lis.len() - 1) as i32);
                while (left <= right) {
                    let mid = left + (right - left) / 2;
                    if (lis[mid as usize] >= nums[i]) {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                lis[left as usize] = nums[i]; // 更新
            }
        }

        lis.len() as i32
    }
}
// @lc code=end

