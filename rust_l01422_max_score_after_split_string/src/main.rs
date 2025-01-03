fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let n = s.len();
        let mut z = 0 as i32; // Zero count on the left side
        let mut o = s.chars().filter(|&c| c == '1').count() as i32; // Total ones in the string
        let mut max = 0 as i32;

        for (i, c) in s.char_indices() {
            if i == n - 1 {
                break;
            }

            match c {
                '0' => z += 1,
                '1' => o -= 1,
                _ => {}
            }

            max = max.max(z + o);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, Solution::max_score(String::from("011101")));
        assert_eq!(5, Solution::max_score(String::from("00111")));
        assert_eq!(3, Solution::max_score(String::from("1111")));
    }
}
