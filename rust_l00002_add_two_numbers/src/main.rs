fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut p = head.as_mut();
        let mut l = l1;
        let mut r = l2;
        let mut carry = 0;
        while l.is_some() || r.is_some() {
            let mut sum = carry;
            if let Some(ll) = l {
                sum += ll.val;
                l = ll.next;
            }
            if let Some(rr) = r {
                sum += rr.val;
                r = rr.next;
            }
            p.next = Some(Box::new(ListNode::new(sum % 10)));
            p = p.next.as_mut().unwrap().as_mut();
            carry = sum / 10;
        }
        if carry > 0 {
            p.next = Some(Box::new(ListNode::new(carry)));
        }
        return head.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l = Some(Box::new(ListNode::new(6)));
        let r = Some(Box::new(ListNode::new(7)));
        let e = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(1))),
        }));
        assert_eq!(Solution::add_two_numbers(l, r), e);
    }
}
