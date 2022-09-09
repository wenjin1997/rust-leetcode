/*
 * @lc app=leetcode id=397 lang=java
 *
 * [397] Integer Replacement
 */

// @lc code=start
class Solution {
    // 当 n 为奇数时，n + 1 或者 n - 1 为偶数。
    // 记忆化搜索
    // 时间复杂度：O(log n)
    // 空间复杂度：O(log n)
    HashMap<Integer, Integer> memo = new HashMap<>();
    public int integerReplacement(int n) {
        memo.put(1, 0);
        if (!memo.containsKey(n)) {
            if (n % 2 == 0) {
                memo.put(n, 1 + integerReplacement(n / 2));
            } else {
                memo.put(n, 2 + Math.min(integerReplacement(n / 2 + 1), integerReplacement((n - 1) / 2)));
            }
        }
        
        return memo.get(n);
    }
}
// @lc code=end

