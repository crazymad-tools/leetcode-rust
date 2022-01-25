/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

/**
 * I      1
 * V      5
 * X      10
 * L      50
 * C      100
 * D      500
 * M      1000
 */
// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut last = 0;
        let mut sum = 0;
        let mut cr: char = '0';
        let size = s.len();

        for ii in 0..size {
            let idx = size - ii - 1;
            cr = s.chars().nth(idx).unwrap();

            if cr == 'I' {
                if last == 10 || last == 5 {
                    sum -= 1;
                    last -= 1;
                } else {
                    sum += 1;
                    last = 1;
                }
            } else if cr == 'V' {
                sum += 5;
                last = 5;
            } else if cr == 'X' {
                if last == 50 || last == 100 {
                    sum -= 10;
                    last -= 10;
                } else {
                    sum += 10;
                    last = 10;
                }
            } else if cr == 'L' {
                sum += 50;
                last = 50;
            } else if cr == 'C' {
                if last == 500 || last == 1000 {
                    sum -= 100;
                    last -= 100;
                } else {
                    sum += 100;
                    last = 100;
                }
            } else if cr == 'D' {
                sum += 500;
                last = 500;
            } else if cr == 'M' {
                sum += 1000;
                last = 1000;
            }
        }

        sum
    }
}

// @lc code=end

