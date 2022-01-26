/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let ss = s.trim();
        let mut flag = 1;
        let mut nums: Vec<u8> = vec![];
        let mut no_zero = false;
        
        let mut ii = 0;
        for cc in ss.bytes() {
            // println!("cc: {}", cc);
            if (cc == 45 || cc == 43) && ii > 0 {
                break;
            } else if cc == 45 {
                flag = -1;
            } else if cc == 43 {
                flag = 1;
            } else if cc < 48 || cc > 48 + 9 {
                break;
            } else if no_zero {
                nums.push(cc - 48);
            } else if !no_zero && cc != 48 {
                no_zero = true;
                nums.push(cc - 48);
            }

            ii = ii + 1;
        }

        let maxn = 0x7fffffff; // 2147483647
        let minn = !maxn; // -2147483648
        let mut sum = 0;

        for ii in 0..nums.len() {
            let ten: i32 = 10;
            let power = (nums.len() - ii - 1) as u32;
            if power > 9 || (power == 9 && nums[ii] > 2) {
                if flag > 0 {
                    return maxn;
                } else {
                    return minn;
                }
            }

            let addon = nums[ii] as i32 * ten.pow(power);

            if flag > 0 {
                if maxn - sum < addon {
                    return maxn;
                }
            } else {
                if minn + sum > -addon {
                    return minn;
                }
            }

            sum += addon;
        }

        return sum * flag;
    }
}
// @lc code=end
