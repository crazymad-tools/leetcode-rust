/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len() as i32;
        let chars: Vec<char> = s.chars().collect();
        let mut maxs1 = [1; 1000];
        let mut maxs2 = [0; 1000];

        for ii in 0..len {
            for offset in 1..len {
                let left_ii = ii - offset;
                let right_ii = ii + offset;

                if left_ii < 0 || right_ii >= len {
                    break;
                }
                if chars[left_ii as usize] == chars[right_ii as usize] {
                    maxs1[ii as usize] = maxs1[ii as usize] + 1;
                } else {
                    break;
                }
            }

            for offset in 0..len {
                let left_ii = ii - offset;
                let right_ii = ii + offset + 1;

                if left_ii < 0 || right_ii >= len {
                    break;
                }
                if chars[left_ii as usize] == chars[right_ii as usize] {
                    maxs2[ii as usize] = maxs2[ii as usize] + 1;
                } else {
                    break;
                }
            }
        }

        // println!("maxs1: {:?}", &maxs1[0..len as usize]);
        // println!("maxs2: {:?}", &maxs2[0..len as usize]);

        let mut max1_idx = 0;
        let mut max2_idx = 0;

        for ii in 0..len as usize {
            if (maxs1[max1_idx] < maxs1[ii]) {
                max1_idx = ii;
            }
            if (maxs2[max2_idx] < maxs2[ii]) {
                max2_idx = ii;
            }
        }

        if maxs1[max1_idx] * 2 - 1 > maxs2[max2_idx] * 2 {
            let offset = maxs1[max1_idx] - 1;
            let begin = max1_idx - offset;
            let end = max1_idx + offset + 1;

            return String::from(&s[begin..end]);
        } else {
            let offset = maxs2[max2_idx];
            let begin = max2_idx - offset + 1;
            let end = max2_idx + offset + 1;

            return String::from(&s[begin..end]);
        }
    }
}

// @lc code=end

