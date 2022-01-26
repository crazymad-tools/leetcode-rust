/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }

        let len = s.len() as i32;
        let chars: Vec<char> = s.chars().collect();
        let mut lines: Vec<String> = Vec::new();
        let mut result: String = String::new();

        for _ in 0..num_rows {
            lines.push(String::from(""));
        }

        for ii in 0..len {
            let mut idx = ii % (num_rows * 2 - 2);
            if idx < num_rows {
                lines[idx as usize].push(chars[ii as usize]);
            } else {
                idx = num_rows - (idx - num_rows) - 2;
                lines[idx as usize].push(chars[ii as usize]);
            }
        }
        for ii in 0..num_rows as usize {
            result.push_str(&lines[ii]);
        }

        result
    }
}

// @lc code=end

