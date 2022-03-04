/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut results: Vec<String> = vec![];

        Solution::get_next(n * 2, String::from(""), 0, &mut results);

        results
    }

    pub fn get_next(n: i32, prefix: String, no_closure: i32, results: &mut Vec<String>) {

        if n == 0 {
            results.push(prefix);
            return;
        }

        if no_closure < n {
            let next_string = prefix.clone() + "(";

            Solution::get_next(n - 1, next_string.clone(), no_closure + 1, results);
        }

        if no_closure > 0 {
            let next_string = prefix.clone() + ")";

            Solution::get_next(n - 1, next_string.clone(), no_closure - 1, results);
        }
    }
}
// @lc code=end

