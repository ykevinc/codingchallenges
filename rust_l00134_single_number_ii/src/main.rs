fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut c1, mut c2) = (0, 0);
        for n in nums {
            c1 = (c1 ^ n) & !c2;
            c2 = (c2 ^ n) & !c1;
        }
        return c1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::single_number(vec![1, 1, 1, 2]));
    }
}
