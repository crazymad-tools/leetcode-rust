/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let mut sum = 0;
        let mut digs: Vec<i32> = vec![];
        let mut origin = x.abs();

        while origin > 0 {
            digs.push(origin % 10);
            origin /= 10;
        }

        let maxn = 0x7fffffff; // 2147483647
        let minn = !maxn; // -2147483648

        for ii in 0..digs.len() {
            let ten: i32 = 10;
            let power = (digs.len() - ii - 1) as u32;
            if power > 9 || (power == 9 && digs[ii] > 2) {
                return 0;
            }

            let addon = digs[ii] * ten.pow(power);

            if x > 0 {
                if maxn - sum < addon {
                    return 0;
                }
            } else {
                if minn + sum > -addon {
                    return 0;
                }
            }

            sum += addon;
        }

        return sum * (x / x.abs());
    }
}

// @lc code=end

