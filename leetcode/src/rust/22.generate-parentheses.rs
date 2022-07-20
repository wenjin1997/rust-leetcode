/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

// @lc code=start
impl Solution {
    // 回溯法
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::back_track(&mut res, n, n, "".to_string());
        res
    }

    pub fn back_track(res: &mut Vec<String>, left: i32, right: i32, sublist: String){
        // 如果左右括号都为0，可以将子列表加入结果中
        if left == 0 && right == 0 {
            res.push(sublist);
            return;
        }

        // 如果剩余左括号还有，就放入左括号
        if left > 0 {
            Self::back_track(res, left - 1, right, sublist.clone() + "(");
        }

        // 如果剩余右括号数量大于剩余左括号数量，就放入右括号
        if right > left {
            Self::back_track(res, left, right - 1, sublist.clone() + ")");
        }
    }
}
// @lc code=end

