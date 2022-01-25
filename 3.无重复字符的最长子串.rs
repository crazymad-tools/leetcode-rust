/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let len = s.len() as i32;
        let mut idx_tags: [i32; 128] = [len + 1; 128];
        let mut maxn: i32 = 1;
        let mut p_left: i32 = 0;
        let mut p_right: i32 = 1;

        let byte_1 = s.bytes().nth(0).unwrap() as usize;

        idx_tags[byte_1] = 0;

        while p_right < len {
            let cur_char = s.bytes().nth(p_right as usize).unwrap() as usize;
            let cur_char_idx = idx_tags[cur_char];

            // println!("cur char: {}, {}, {}", cur_char, cur_char_idx, maxn);

            if cur_char_idx < len + 1 {
                let new_p_left = (cur_char_idx) + 1;

                for ii in p_left..((cur_char_idx + 1) as i32) {
                    idx_tags[s.bytes().nth(ii as usize).unwrap() as usize] = len + 1;
                }

                p_left = new_p_left as i32;
                idx_tags[cur_char] = p_right;
            } else {
                idx_tags[cur_char as usize] = p_right;
            }

            p_right += 1;

            if p_right - p_left as i32 > maxn {
                maxn = p_right - p_left;
            }
        }

        maxn
    }
}
// @lc code=end

