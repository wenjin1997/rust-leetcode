/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    // 栈
    pub fn is_valid(s: String) -> bool {
        // 事先放入一个元素，如果第一个字符为 ']'，则栈不为空，可以进行比较
        let mut stack = vec!['0'];
        for c in s.chars() {
            match c {
                // 左括号进栈
                '(' | '[' | '{' => { stack.push(c); },
                // 右括号进行比较
                ')' => {if stack.pop().unwrap() != '(' { return false; }},
                ']' => {if stack.pop().unwrap() != '[' { return false; }},
                '}' => {if stack.pop().unwrap() != '{' { return false; }},
                _ => (),
            }
        }
        stack.len() == 1
    }    
}
// @lc code=end

