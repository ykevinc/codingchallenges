fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut c = Self::count_digits(x);

        let mut m = x;
        while c > 1 {
            let r = m % 10;
						let p = i32::pow(10, (c - 1) as u32);
            let l = m / p;
            if r != l {
                return false;
            }
            m -= l * p;
            m /= 10;
            c -= 2;
        }

        return true;
    }

    fn count_digits(x: i32) -> i32 {
        let mut c = 0;
        let mut m = x;
        while m > 0 {
            m /= 10;
            c += 1;
        }
        return c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test99000() {
        assert_eq!(false, Solution::is_palindrome(99000));
    }

    #[test]
    fn test88() {
        assert_eq!(true, Solution::is_palindrome(88));
    }

    #[test]
    fn test777() {
        assert_eq!(true, Solution::is_palindrome(777));
    }

    #[test]
    fn test656() {
        assert_eq!(true, Solution::is_palindrome(656));
    }

    #[test]
    fn test443() {
        assert_eq!(false, Solution::is_palindrome(443));
    }

    #[test]
    fn test0() {
        assert_eq!(true, Solution::is_palindrome(0));
    }
}
