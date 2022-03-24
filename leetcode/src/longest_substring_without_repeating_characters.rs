// Source : https://leetcode.com/problems/longest-substring-without-repeating-characters/
// Author : eryk xu
// Date   : 2022-03-25

use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut count = 0;
        let mut result = 0;
        let chars = s.as_bytes();
        let mut char_map = HashMap::new();
        while start < s.len() {
            for i in start..s.len() {
                match char_map.get(chars.get(i).unwrap()) {
                    None => {
                        count += 1;
                        char_map.insert(chars.get(i).unwrap(), i);
                    }
                    Some(p) => {
                        if result < count {
                            result = count;
                        }
                        count = 0;
                        start = p + 1;
                        char_map.clear();
                        break;
                    }
                }
            }
        }
        result
    }

    #[allow(dead_code)]
    fn length_of_longest_substring1(s: String) -> i32 {
        let mut m = HashMap::new();
        let mut ans = 0;
        let mut before = -1;
        let mut current = 0;
        for c in s.chars() {
            if let Some(last) = m.insert(c, current) {
                before = max(before, last)
            }
            ans = max(ans, current - before);
            current += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_substring_without_repeating_characters::Solution;

    #[test]
    fn test() {
        struct TestCase {
            s: String,
            except: i32,
        }

        let test_case = vec![
            TestCase {
                s: "abcabcbb".to_string(),
                except: 3,
            },
            TestCase {
                s: "bbbbb".to_string(),
                except: 1,
            },
            TestCase {
                s: "pwwkew".to_string(),
                except: 3,
            },
            TestCase {
                s: "ohomm".to_string(),
                except: 3,
            },
        ];

        for TestCase { s, except } in test_case.into_iter() {
            assert_eq!(Solution::length_of_longest_substring(s), except);
        }
    }
}
