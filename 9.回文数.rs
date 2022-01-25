/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else if x < 10 {
            return true;
        }

        let mut num = x;
        let mut nums = vec![];

        while num >= 10 {
            nums.push(num % 10);
            num /= 10;
        }
        nums.push(num);

        for ii in 0..(nums.len() / 2) {
            if nums[ii] != nums[nums.len() - ii - 1] {
                return false;
            }
        }

        true
    }
}
// @lc code=end

