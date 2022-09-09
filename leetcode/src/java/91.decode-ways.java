/*
 * @lc app=leetcode id=91 lang=java
 *
 * [91] Decode Ways
 */

// @lc code=start
class Solution {
    // 动态规划
    public int numDecodings(String s) {
        int n = s.length();
        int[] dp = new int[n + 1];
        dp[0] = 1; // 空字符串有唯一的解码方法

        for (int i = 1; i <= n; i++) {
            // 如果当前字符不为 '0'，说明可以解码为单个字符
            if (s.charAt(i - 1) != '0') {
                dp[i] += dp[i - 1];
            }

            // 可以组成两个数字
            if (i > 1 && s.charAt(i - 2) != '0' && ((s.charAt(i - 2) - '0') * 10 + (s.charAt(i - 1) - '0')) <= 26 ) {
                dp[i] += dp[i - 2];
            }
        }

        return dp[n];
    }
}
// @lc code=end

