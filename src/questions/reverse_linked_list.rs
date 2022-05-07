struct Solution;

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut backward_node: Option<Box<ListNode>> = None; // points to the last visited node
        let mut forward_node: Option<Box<ListNode>> = head;

        while let Some(mut temp_node) = forward_node {
            // iterating forward through the original linkedlist
            forward_node = temp_node.next;

            // reversing the pointer to the next (starting with None)
            temp_node.next = backward_node;

            // stores modified (reversed) node
            backward_node = Some(temp_node);
        }
        backward_node
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;
    #[test]
    fn test_example_1() {
        // link list [4,3,2,1]
        let mut expected_link_node = ListNode::new(4);
        for i in [3, 2, 1] {
            let mut new_link_node = ListNode::new(i);
            new_link_node.next = Some(Box::new(expected_link_node));
            expected_link_node = new_link_node
        }

        // link list [1,2,3,4]
        let mut current_link_node = ListNode::new(1);
        for i in [2, 3, 4] {
            let mut new_link_node = ListNode::new(i);
            new_link_node.next = Some(Box::new(current_link_node));
            current_link_node = new_link_node
        }

        assert_eq!(
            Some(Box::new(expected_link_node)),
            Solution::reverse_list(Some(Box::new(current_link_node)))
        );
    }
}
