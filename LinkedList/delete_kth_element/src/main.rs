#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
    fn delete_kth(mut head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
        if k == 0 {
            return head.and_then(|node| node.next);
        }

        let mut current = &mut head;
        for _ in 0..k - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return head;
            }
        }

        if let Some(node) = current {
            node.next = node.next.as_mut().and_then(|next| next.next.take());
        }

        head
    }

    fn print_list(head: &Option<Box<ListNode>>) {
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            print!("{} -> ", node.val);
            curr = node.next.as_ref();
        }
        println!("None");
    }
}

fn main() {
    let mut head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        })),
    }));

    println!("Original list:");
    ListNode::print_list(&head);
    head = ListNode::delete_kth(head, 2);

    println!("After deleting 2nd element:");
    ListNode::print_list(&head);
}
