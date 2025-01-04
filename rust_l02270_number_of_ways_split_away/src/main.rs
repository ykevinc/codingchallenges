fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut l: i64 = 0;
        let mut r: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut c = 0;
        for (i, &n) in nums.iter().enumerate() {
            if i == nums.len() - 1 {
                break;
            }
            l += n as i64;
            r -= n as i64;
            if l >= r {
                c += 1;
            }
        }
        return c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::ways_to_split_array(vec![10, 4, -8, 7]));
        assert_eq!(2, Solution::ways_to_split_array(vec![2, 3, 1, 0]));
    }
}
