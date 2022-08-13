/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */

// @lc code=start
impl Solution {
    /// 双指针
    /// 两个指针一个在开始，一个在结尾，然后交换它们的字符
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut start, mut end) = (0, s.len() - 1);
        while start < end {
            // 交换字符
            let temp = s[start];
            s[start] = s[end];
            s[end] = temp;
            start += 1;
            end -= 1;
        }
    }
}
// @lc code=end

