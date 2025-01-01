fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0 as usize, height.len() as usize - 1);
        let mut max = 0;
        while l < r {
            max = std::cmp::max(std::cmp::min(height[l], height[r]) * ((r - l) as i32), max);
            match height[l].cmp(&height[r]) {
                std::cmp::Ordering::Less => l += 1,
                _ => r -= 1,
            }
        }
        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]))
    }
}
