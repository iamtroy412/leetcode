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

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Since we know both lists are sorted, to merge them we need to compare
        // the values at the current ListNode, take the lower value, progress
        // that list to the next node while keeping the other list at it's same node.
        // Repeat the process (recursive)

        // Use `match` to cover the base cases of both or one of the lists
        // being empty. They are Options so will either return `Some` or `None`
        match (list1, list2) {
            // Both lists are empty, return None
            (None, None) => None,
            // Only one of the lists has data, return it
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            // Both lists contain data
            // Use `match` to compare the values of the current ListNodes
            // If the ListNode value of the first list is greater than or
            // equal to the second lists value, return a new ListNode with
            // the smaller value (the second lists value) and it's next node
            // pointing to the result of merging the first list starting at it's
            // current node, and the second list starting at it's next node.
            // (ie advance the second list, keep the first list as is)
            (Some(l1), Some(l2)) => match l1.val >= l2.val {
                true => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next),
                })),
                // The value of the first list is less than the value of the
                // second list, return a new ListNode with
                // the smaller value (the first lists value) and it's next node
                // pointing to the result of merging the second list starting at it's
                // current node, and the first list starting at it's next node.
                // (ie advance the first list, keep the second list as is)
                // opposite of above
                false => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
            },
        }
    }
}
