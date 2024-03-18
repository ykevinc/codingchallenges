use std::cmp;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut bytes = s.as_bytes();
        let (mut l, mut r) = (0, 0);
        let mut v = HashSet::new();
        while r < s.len() {
            println!("{} {} {} {} {:?}", s, l, r, &s[l..=r], v);
            while v.contains(&bytes[r]) {
                v.remove(&bytes[l]);
                l += 1;
            }
            v.insert(bytes[r]);
            max = cmp::max(max, r - l + 1);
            r += 1;
        }
        return max as i32;
    }

    pub fn length_of_longest_substring_n2(s: String) -> i32 {
        let mut max = 0;
        let bytes = s.as_bytes();
        'l: for i in 0..s.len() {
            let mut v = HashSet::new();
            for j in i + 1..=s.len() {
                if !v.insert(bytes[j - 1]) {
                    continue 'l;
                }
                max = cmp::max(max, j - i);
            }
        }
        return max as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_longest_substring("ABC".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("AAA".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }
}
