use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Link<T>,
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

impl<T> DoublyLinkedList<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        DoublyLinkedList { head: None, tail: None }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(prev) = old_tail.borrow().prev.as_ref().and_then(|w| w.upgrade()) {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head.take();
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn display_forward(&self) {
        let mut node = self.head.clone();
        while let Some(current) = node {
            print!("{:?} <-> ", current.borrow().value);
            node = current.borrow().next.clone();
        }
        println!("None");
    }

    pub fn display_backward(&self) {
        let mut node = self.tail.clone();
        while let Some(current) = node {
            print!("{:?} <-> ", current.borrow().value);
            node = current.borrow().prev.as_ref().and_then(|w| w.upgrade());
        }
        println!("None");
    }
}

fn main() {
    let mut dll = DoublyLinkedList::new();

    dll.push_back(10);
    dll.push_front(20);
    dll.push_back(30);
    dll.display_forward();
    dll.display_backward();

    dll.pop_front();
    dll.display_forward();

    dll.pop_back();
    dll.display_forward();
}
