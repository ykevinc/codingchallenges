use std::cmp;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut v = Vec::new();
        let (mut l, mut r) = (nums1.iter(), nums2.iter());
        let mut ll = l.next();
        let mut rr = r.next();
        loop {
            if ll.is_some() && rr.is_some() {
                if ll.unwrap() < rr.unwrap() {
                    v.push(ll.unwrap());
                    ll = l.next();
                } else if ll.unwrap() > rr.unwrap() {
                    v.push(rr.unwrap());
                    rr = r.next();
                } else {
                    v.push(ll.unwrap());
                    v.push(rr.unwrap());
                    ll = l.next();
                    rr = r.next();
                }
            } else if ll.is_some() {
                v.push(ll.unwrap());
                ll = l.next();
            } else if rr.is_some() {
                v.push(rr.unwrap());
                rr = r.next();
            } else {
                break;
            }
        }
        let m = v.len() / 2;
        if v.len() % 2 == 0 {
            (v[m] + v[m - 1]) as f64 / 2.0
        } else {
            return *v[m] as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 4], vec![2, 5]),
            3.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 4], vec![2]),
            2.0
        );
    }
}
