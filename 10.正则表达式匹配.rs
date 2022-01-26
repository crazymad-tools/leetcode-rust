/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */

// @lc code=start
struct Pattern {
    content: char,
    p_type: i32,
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // .
        // n
        // .*
        // n*

        let p_chars: Vec<char> = p.chars().collect();
        let s_chars: Vec<char> = s.chars().collect();
        let mut patterns: Vec<Pattern> = Vec::new();

        for ii in 0..p_chars.len() {
            let cc = p_chars[ii];

            if cc == '.' || (cc <= 'z' && cc >= 'a') {
                patterns.push(Pattern {
                    content: cc,
                    p_type: 0,
                })
            } else if p_chars[ii] == '*' {
                let latest = patterns.len() - 1;
                let pattern = &mut patterns[latest];
                pattern.p_type = 1;
            }
        }

        let mut p_idx = 0;
        let mut s_idx = 0;
        let mut matched = 0;

        Solution::match_next(p_idx, s_idx, &s_chars, &patterns, matched)
    }

    fn match_next(
        p_idx: usize,
        s_idx: usize,
        chars: &Vec<char>,
        patterns: &Vec<Pattern>,
        matched: i32,
    ) -> bool {
        if p_idx >= patterns.len() && s_idx >= chars.len() {
            return true;
        } else if p_idx >= patterns.len() {
            return false;
        }

        let mut cc = 0 as char;
        if (s_idx < chars.len()) {
            cc = chars[s_idx];
        }
        let pattern = &patterns[p_idx];

        if cc != 0 as char && (cc == pattern.content || pattern.content == '.') {
            if pattern.p_type == 0 {
                return Solution::match_next(p_idx + 1, s_idx + 1, chars, patterns, 0);
            } else if pattern.p_type == 1 {
                let res1 = Solution::match_next(p_idx + 1, s_idx, chars, patterns, 0);
                let res2 = Solution::match_next(p_idx, s_idx + 1, chars, patterns, matched + 1);

                return res1 | res2;
            }
        } else if pattern.p_type == 1 && matched >= 0 {
            return Solution::match_next(p_idx + 1, s_idx, chars, patterns, 0);
        }

        false
    }
}
// @lc code=end

