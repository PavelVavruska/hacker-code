use std::collections::vec_deque::VecDeque;
struct Solution;

//Definition for singly-linked list.
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut current_node = head;
        let mut number_of_nodes = 0;
        let mut deque: VecDeque<i32> = VecDeque::new();

        // calculate number of nodes
        loop {
            let list_node = match current_node {
                Some(node) => node,
                None => break,
            };
            number_of_nodes += 1;
            deque.push_back(list_node.val);

            current_node = list_node.next;
        }

        // compare ends of deque
        for _ in 0..number_of_nodes / 2 {
            if deque.pop_front() != deque.pop_back() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;
    #[test]
    fn test_example_1() {
        // link list [1,2,2,1]
        let mut current_link_node = ListNode::new(1);
        for i in [2, 2, 1] {
            let mut new_link_node = ListNode::new(i);
            new_link_node.next = Some(Box::new(current_link_node));
            current_link_node = new_link_node
        }

        assert_eq!(
            true,
            Solution::is_palindrome(Some(Box::new(current_link_node)))
        );
    }

    #[test]
    fn test_example_2() {
        // link list [1,2,3,2,1]
        let mut current_link_node = ListNode::new(1);
        for i in [2, 3, 2, 1] {
            let mut new_link_node = ListNode::new(i);
            new_link_node.next = Some(Box::new(current_link_node));
            current_link_node = new_link_node
        }

        assert_eq!(
            true,
            Solution::is_palindrome(Some(Box::new(current_link_node)))
        );
    }

    #[test]
    fn test_example_3() {
        // link list [1,2,3,1]
        let mut current_link_node = ListNode::new(1);
        for i in [2, 3, 1] {
            let mut new_link_node = ListNode::new(i);
            new_link_node.next = Some(Box::new(current_link_node));
            current_link_node = new_link_node
        }

        assert_eq!(
            false,
            Solution::is_palindrome(Some(Box::new(current_link_node)))
        );
    }
}
