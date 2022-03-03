use std::{cmp, iter::FromIterator};

struct Solution {}

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

fn main() {
    let s = String::from("([[]]");

    let res = Solution::is_valid(s);

    println!("{:?}", res);
}
