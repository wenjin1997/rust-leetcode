/*
 * @lc app=leetcode id=541 lang=java
 *
 * [541] Reverse String II
 */

// @lc code=start
class Solution {
    public String reverseStr(String s, int k) {
        char[] ch = s.toCharArray();
        int n = s.length();
        for (int i = 0; i < n; i = i + 2 * k) {
            // 剩余字符小于 2k 并且大于等于 k，则反转前 k 个字符
            if (i + k < n) {
                reverse(ch, i, i + k - 1);
                continue;
            }
            // 如果剩余字符小于 k，则反转所有字符
            reverse(ch, i, n - 1);
        }
        return new String(ch);
    }

    private void reverse(char[] ch, int i, int j) {
        for (; i < j; i++, j--) {
            char temp = ch[i];
            ch[i] = ch[j];
            ch[j] = temp;
        }
    }
}
// @lc code=end

