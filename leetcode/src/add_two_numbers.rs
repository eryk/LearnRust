// Source : https://leetcode.com/problems/add-two-numbers/
// Author : eryk xu
// Date   : 2022-03-24

// Definition for singly-linked list.
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

    fn new_list(nodes: Vec<i32>) -> Option<Box<ListNode>> {
        match nodes.len() {
            0 => None,
            _ => {
                let mut head = Some(Box::new(ListNode::new(*nodes.first().unwrap())));
                let mut tail = &mut head;
                for i in 1..nodes.len() {
                    let node = Some(Box::new(ListNode::new(*nodes.get(i).unwrap())));
                    tail.as_deref_mut().unwrap().next = node;
                    tail = &mut tail.as_deref_mut().unwrap().next;
                }
                head
            }
        }
    }
}

struct Solution;

// type BoxNode = Option<Box<ListNode>>;

impl Solution {
    #[allow(unused_assignments)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;

        let mut carry = 0;
        while p1.is_some() | p2.is_some() {
            let mut v1 = 0;
            let mut v2 = 0;
            let mut val = 0;

            if p1.is_some() {
                v1 = p1.unwrap().val;
                p1 = p1.unwrap().next.as_ref();
            }
            if p2.is_some() {
                v2 = p2.unwrap().val;
                p2 = p2.unwrap().next.as_ref();
            }
            val = carry + v1 + v2;
            let node = Some(Box::new(ListNode::new(val % 10)));
            tail.as_deref_mut().unwrap().next = node;
            tail = &mut tail.as_deref_mut().unwrap().next;
            carry = val / 10;
        }
        if carry == 1 {
            tail.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        head.unwrap().next
    }

    #[allow(dead_code)]
    pub fn add_two_numbers1(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::merge(l1, l2, 0, ListNode::new(-1))
    }

    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
        mut val: i32,
        mut ln: ListNode,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && val == 0 {
            return None;
        }
        if let Some(n1) = l1 {
            val += n1.val;
            l1 = n1.next;
        };
        if let Some(n2) = l2 {
            val += n2.val;
            l2 = n2.next;
        };
        ln.val = if val > 9 { val - 10 } else { val };
        ln.next = Solution::merge(l1, l2, val / 10, ListNode::new(-1));
        Some(Box::new(ln))
    }

    #[allow(dead_code)]
    pub fn add_two_numbers2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers2(n1.next, n2.next),
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers2(
                            Solution::add_two_numbers2(carry, n1.next),
                            n2.next,
                        ),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers::{ListNode, Solution};

    #[test]
    fn test() {
        struct TestCase {
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
            except: Option<Box<ListNode>>,
        }

        let test_case = vec![
            TestCase {
                l1: ListNode::new_list(vec![2, 4, 3]),
                l2: ListNode::new_list(vec![5, 6, 4]),
                except: ListNode::new_list(vec![7, 0, 8]),
            },
            TestCase {
                l1: ListNode::new_list(vec![9, 9, 9, 9, 9, 9, 9]),
                l2: ListNode::new_list(vec![9, 9, 9, 9]),
                except: ListNode::new_list(vec![8, 9, 9, 9, 0, 0, 0, 1]),
            },
        ];

        for TestCase { l1, l2, except } in test_case.into_iter() {
            assert_eq!(Solution::add_two_numbers(l1, l2), except)
        }
    }
}
