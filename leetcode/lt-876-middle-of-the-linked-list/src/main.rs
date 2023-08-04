// Definition for singly-linked list.
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

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut size = 0;
    match head {
        Some(head_node) => {
            size += 1;
            let mut cur = head_node.clone();
            loop {
                match cur.next {
                    Some(x) => {
                        cur = x;
                        size += 1
                    }
                    None => {
                        break;
                    }
                }
            }
            let middle = ((size / 2) as f64).ceil() as i32; 
            cur = head_node;
            for _ in 0..middle {
                match cur.next {
                    Some(x) => {
                        cur = x;
                    }
                    None => {}
                }
            }
            Some(cur)
        }
        None => head,
    }
}

// Implemented from memory after looking this solution
// https://leetcode.com/problems/middle-of-the-linked-list/solutions/3334252/java-rust-python-two-pointers-solution-with-clear-explanation-and-real-life-application/
fn middle_node_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow.clone()
}

fn main() {
    let mut ll1 = ListNode::new(1);
    let mut ll2 = ListNode::new(2);
    let mut ll3 = ListNode::new(3);
    let mut ll4 = ListNode::new(4);
    let ll5 = ListNode::new(5);
    ll4.next = Some(Box::new(ll5));
    ll3.next = Some(Box::new(ll4));
    ll2.next = Some(Box::new(ll3));
    ll1.next = Some(Box::new(ll2));
    println!("{:?}", middle_node(Some(Box::new(ll1.clone()))));
    println!("{:?}", middle_node_v2(Some(Box::new(ll1))));
}
