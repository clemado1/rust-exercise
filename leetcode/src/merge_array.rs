
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    match (l1, l2) {
        (Some(mut x), Some(mut y)) => {
            if x.val > y.val {
                y.next = y.next.and_then(|n| merge_two_lists(Some(x), Some(n)));
                Some(y)
            } else {
                x.next = x.next.and_then(|n| merge_two_lists(Some(n), Some(y)));
                Some(y)
            }
        }
        (None, None) => None,
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
    }

}

fn main() {
    let mut v1 = Some(Box::new(ListNode::new(1)));
    v1.unwrap().next = Some(Box::new(ListNode::new(2)));
    v1 = v1.unwrap().next;
    v1.unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut v2 = Some(Box::new(ListNode::new(1)));
    v2.unwrap().next = Some(Box::new(ListNode::new(3)));
    v2 = v2.unwrap().next;
    v2.unwrap().next = Some(Box::new(ListNode::new(4)));

    merge_two_lists(v1, v2);
}