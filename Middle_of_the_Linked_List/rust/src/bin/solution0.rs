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

struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Init counter for the total length of the linked list.
        let mut len = 0;

        // Clone the current head node to a mutable variable so we can
        // continue to update it to the next node in the linked list
        let mut current = head.clone();

        // While `current` continues to destructure into `Some` `Box`,
        // progress to the next node in the linked list, and add 1
        // to the counter for the length of the linked list.
        // When `current` becomes `None`, we know we're reached the
        // end of the linked list.
        while let Some(node) = current {
            current = node.next;
            len += 1;
        }

        // Now that we know the size (length) of the linked list,
        // calculate the middle of it, and take a new mutable copy
        // of the head node to walk down the linked list.
        let mut node = head.clone();
        let middle = len / 2;

        // Starting at 0, up to the middle of the linked list,
        // step the current node to the next node in the list.
        // We are `unwrap`ing the option here since there should
        // always be a node until the end of the list, and we are
        // only going to the middle.
        for n in 0..middle {
            node = node.unwrap().next;
        }

        // returns `Option<Box<ListNode>>`
        node
    }
}
