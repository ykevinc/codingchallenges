use std::collections::{HashMap};
use std::collections::hash_map::Entry::{Occupied,Vacant};

fn main() {
    println!("Hello, world!");
}


struct Solution {
}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let mut m: HashMap<i32,usize> = HashMap::new();
	for (i,n) in nums.iter().enumerate() {
		match m.entry(target-n) {
			Occupied(e) => return vec![*(e.get()) as i32,i as i32],
			Vacant(_e) => m.insert(*n, i),
		};
		
	}
	return vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_rating() {
	assert_eq!(Solution::two_sum(vec![], 3), vec![]);
	assert_eq!(Solution::two_sum(vec![2,1], 3), vec![0,1])

    }
}
