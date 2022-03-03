/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let ss = s;
        let ss = ss.replace("()", "");
        let ss = ss.replace("[]", "");
        let ss = ss.replace("{}", "");

        let mut prefix_stack: Vec<char> = vec![];

        for cc in ss.chars() {
            let len = prefix_stack.len();

            // if len > ss.len() / 2 {
            //     break;
            // }
            let mut last_char = 'a';
            if len > 0 {
                last_char = prefix_stack[len - 1]
            }

            if (last_char == '[' && cc == ']')
                || (last_char == '(' && cc == ')')
                || (last_char == '{' && cc == '}')
            {
                prefix_stack.pop();
            } else {
                prefix_stack.push(cc);
            }
        }

        prefix_stack.len() == 0
    }
}

// @lc code=end

