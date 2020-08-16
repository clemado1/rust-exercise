fn main() {
    for i in 0..5 {
        println!("i: {}", i);
    }
}

// Definition for singly-linked list.
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

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });

    let mut last = dummy.clone();
    let mut inval = dummy.as_mut();
    let mut count = 0;
    
    while last.next.is_some() {
        count += 1;
        last = last.next.unwrap();

        if count > n {
            inval = inval.next.as_mut().unwrap();
        }
    }

    inval.next = inval.next.as_mut().unwrap().next.clone();

    dummy.next

}