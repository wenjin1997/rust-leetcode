/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
impl Solution {
    /// 哈希表
    /// 由于只含有26个字母，因此可以用数组来记录字母出现的次数。
    pub fn is_anagram(s: String, t: String) -> bool {
        let (mut count_s, mut count_t) = (vec![0; 26], vec![0; 26]);

        // 记录字符串 s 中各个字母出现的次数
        for b in s.into_bytes() {
            count_s[(b - b'a') as usize] += 1;
        }

        // 记录字符串 t 中各个字母出现的次数
        for b in t.into_bytes() {
            count_t[(b - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if count_s[i] != count_t[i] { return false; }
        }

        true
    }
}
// @lc code=end

