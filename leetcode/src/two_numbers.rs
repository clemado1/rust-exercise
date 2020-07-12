fn main() {
    let mut l1 = ListNode::new(2);
    let mut l2 = ListNode::new(4);
    let mut l3 = ListNode::new(5);

    l2.next = Some(Box::new(l3));
    l1.next = Some(Box::new(l2));

    let mut r1 = ListNode::new(5);
    let mut r2 = ListNode::new(6);

    r1.next = Some(Box::new(r2));

    let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(r1)));

    println!("{:?}", result.unwrap());
}

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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_recursive(l1, l2, 0)
}

pub fn add_recursive(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    let next: Option<Box<ListNode>> = match (&l1, &l2) {
        (None, None) => {
            if carry != 0 {
                Some(Box::new(ListNode::new(carry)))
            } else {
                None
            }
        }
        (_, _) => {
            let r1 = l1.unwrap_or(Box::new(ListNode::new(0)));
            let r2 = l2.unwrap_or(Box::new(ListNode::new(0)));

            let val_sum = r1.val + r2.val + carry;
            let mut l3 = ListNode::new(val_sum % 10);

            l3.next = add_recursive(r1.next, r2.next, val_sum / 10);

            Some(Box::new(l3))
        }
    };

    next
}
