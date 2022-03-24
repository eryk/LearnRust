// Source : https://leetcode.com/problems/two-sum/
// Author : eryk xu
// Date   : 2022-03-24

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if map.contains_key(&(target - n)) {
                return vec![*map.get(&(target - n)).unwrap(),i as i32];
            } else {
                map.insert(n, i as i32);
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::two_sum::Solution;

    #[test]
    fn test() {
        struct TestCase {
            nums: Vec<i32>,
            target: i32,
            except: Vec<i32>,
        }

        let test_case = vec![
            TestCase {
                nums: vec![2, 7, 11, 15],
                target: 9,
                except: vec![0, 1],
            },
            TestCase {
                nums: vec![3, 2, 4],
                target: 6,
                except: vec![1, 2],
            },
            TestCase {
                nums: vec![3, 3],
                target: 6,
                except: vec![0, 1],
            },
        ];
        for TestCase { nums, target, except } in test_case.iter() {
            assert_eq!(Solution::two_sum(nums.to_vec(), *target), except.to_vec())
        }
    }
}