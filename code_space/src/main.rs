struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::node_plus(l1, l2, 0)
    }

    pub fn node_plus(
        node1: Option<Box<ListNode>>,
        node2: Option<Box<ListNode>>,
        carray: i32,
    ) -> Option<Box<ListNode>> {
        if let None = node1 {
            if let None = node2 {
                if carray > 0 {
                    return Some(Box::new(ListNode {
                        val: carray,
                        next: None
                    }))
                }

                return None;
            }
        }

        let mut next1: Option<Box<ListNode>> = None;
        let mut next2: Option<Box<ListNode>> = None;
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;


        match node1 {
            Some(node) => {
                num1 = node.val;
                next1 = node.next;
            }
            None => {
                next1 = None;
                num1 = 0;
            }
        }

        match node2 {
            Some(node) => {
                num2 = node.val;
                next2 = node.next;
            }
            None => {
                next2 = None;
                num2 = 0;
            }
        }

        let sum: i32 = (num1 + num2 + carray) % 10;
        let new_carray: i32 = (num1 + num2 + carray) / 10;
        // println!("{}, {}, {}, {}, {}", num1, num2, sum, carray, new_carray);

        Some(Box::new(ListNode {
            val: sum,
            next: Solution::node_plus(next1, next2, new_carray),
        }))
    }
}

fn getNodeList(nums: Vec<i32>) -> Option<Box<ListNode>> {
    if nums.len() == 0 {
        return None;
    }

    let next = getNodeList(nums[1..nums.len()].to_vec());

    Some(Box::new(ListNode {
        val: nums[0],
        next: next,
    }))
}

fn getArrayFromNodeList(list: Option<Box<ListNode>>) {

    match list {
        Some(node) => {
            print!("{},", node.val);

            getArrayFromNodeList(node.next);
        }
        None => {},
    };
}

fn main() {
    let arr1 = vec![2,4,9];
    let arr2 = vec![5,6,4,9];

    let nodes1 = getNodeList(arr1);
    let nodes2 = getNodeList(arr2);

    // println!("{:?}", nodes1);

    let sum = Solution::add_two_numbers(nodes1, nodes2);
    getArrayFromNodeList(sum);
    // println!("{:?}", sum);
}
