/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    // 动态规划
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let n = s.len();
        let s_bytes = s.as_bytes();
        // 对角线要初始化为 true， 这里整体都初始化为 true 了
        let mut dp = vec![vec![true; n]; n];
        // 记录结果，第一个是最长子串的起始下标，第二个是最长子串的末尾下标
        let (mut index, mut max_len) = (0, 1); 
        
        // 采用列升序，行升序的方式进行动态转移
        for j in 1..n {
            for i in 0..j {
                if s_bytes[i] != s_bytes[j] {
                    dp[i][j] = false;
                } else {
                    if (j - i < 3) {
                        dp[i][j] = true;
                    } else { // 进行动态转移
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                }

                // 如果当前子串是回文子串并且当前子串的长度大于记录的长度，则更新数据
                if dp[i][j] && ((j - i + 1) > max_len) {
                    index = i;
                    max_len = j - i + 1;
                }
            }
        }
        s[index as usize .. (index + max_len) as usize].to_string()
    }
}
// @lc code=end

