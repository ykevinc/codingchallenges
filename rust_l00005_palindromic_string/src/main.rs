fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let a = s.as_bytes();
        let (e, es) = Solution::longest_palindrome_offset(a, false);
        let (o, os) = Solution::longest_palindrome_offset(a, true);
        return String::from_utf8(if e > o { es } else { os }.to_vec()).unwrap();
    }

    fn longest_palindrome_offset(s: &[u8], offset: bool) -> (i32, &[u8]) {
        let (mut m, mut w): (i32, &[u8]) = (0, b"");
        for i in 0..=s.len() {
            let (mut l, mut r): (i32, i32) = (i as i32, i as i32);
            if offset {
                r += 1;
            }
            while l >= 0 && (r as usize) < s.len() && s[l as usize] == s[r as usize] {
                if r - l + 1 > m {
                    m = r - l + 1;
                    w = &s[l as usize..(r + 1) as usize];
                }
                l -= 1;
                r += 1;
            }
        }
        return (m, w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_palindrome("aba".to_string()),
            "aba".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("ab".to_string()),
            "a".to_string()
        );
    }
}
