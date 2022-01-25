/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for ii in 0..(nums.len() - 1) {
            // println!("{}", ii);
            let num1 = nums[ii];

            for jj in (ii + 1)..nums.len() {
                let num2 = nums[jj];
                if (num2 + num1 == target) {
                    return vec![ii:usize as i32, jj:usize as i32];
                }
            }
        }

        return nums;
    }
}
// @lc code=end

