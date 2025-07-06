use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    count: i32,
    keys: HashSet<String>,
    prev: Option<Weak<RefCell<Node>>>,
    next: Link,
}

impl Node {
    fn new(count: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            count,
            keys: HashSet::new(),
            prev: None,
            next: None,
        }))
    }
}

pub struct AllOne {
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
    map: HashMap<String, Rc<RefCell<Node>>>,
}

impl AllOne {
    pub fn new() -> Self {
        let head = Node::new(i32::MIN);
        let tail = Node::new(i32::MAX);
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));
        Self { head, tail, map: HashMap::new() }
    }

    pub fn inc(&mut self, key: String) {
        if let Some(node) = self.map.get(&key).cloned() {
            let cnt = node.borrow().count;
            let next = node.borrow().next.clone().unwrap();
            let new_node = if next.borrow().count == cnt + 1 {
                next
            } else {
                let new = Node::new(cnt + 1);
                Self::insert_after(&node, &new);
                new
            };
            new_node.borrow_mut().keys.insert(key.clone());
            self.map.insert(key.clone(), new_node);
            node.borrow_mut().keys.remove(&key);
            if node.borrow().keys.is_empty() {
                Self::remove(&node);
            }
        } else {
            let first = self.head.borrow().next.clone().unwrap();
            let new_node = if first.borrow().count == 1 {
                first
            } else {
                let new = Node::new(1);
                Self::insert_after(&self.head, &new);
                new
            };
            new_node.borrow_mut().keys.insert(key.clone());
            self.map.insert(key, new_node);
        }
    }

    pub fn dec(&mut self, key: String) {
        if let Some(node) = self.map.get(&key).cloned() {
            let cnt = node.borrow().count;
            node.borrow_mut().keys.remove(&key);
            if cnt == 1 {
                self.map.remove(&key);
            } else {
                let prev = node.borrow().prev.as_ref().unwrap().upgrade().unwrap();
                let new_node = if prev.borrow().count == cnt - 1 {
                    prev
                } else {
                    let new = Node::new(cnt - 1);
                    Self::insert_after(&prev, &new);
                    new
                };
                new_node.borrow_mut().keys.insert(key.clone());
                self.map.insert(key, new_node);
            }
            if node.borrow().keys.is_empty() {
                Self::remove(&node);
            }
        }
    }

    pub fn get_max_key(&self) -> String {
        let mut node = self.tail.borrow().prev.as_ref().unwrap().upgrade().unwrap();
        while node.borrow().count != i32::MIN && node.borrow().keys.is_empty() {
            let prev = node.borrow().prev.as_ref().unwrap().upgrade().unwrap();
            node = prev;
        }
        let result = node.borrow().keys.iter().next().cloned().unwrap_or_default();
        result
    }

    pub fn get_min_key(&self) -> String {
        let mut node = self.head.borrow().next.as_ref().unwrap().clone();
        while node.borrow().count != i32::MAX && node.borrow().keys.is_empty() {
            let next = node.borrow().next.as_ref().unwrap().clone();
            node = next;
        }
        let result = node.borrow().keys.iter().next().cloned().unwrap_or_default();
        result
    }

    fn insert_after(prev: &Rc<RefCell<Node>>, node: &Rc<RefCell<Node>>) {
        let next = prev.borrow().next.clone().unwrap();
        node.borrow_mut().prev = Some(Rc::downgrade(prev));
        node.borrow_mut().next = Some(next.clone());
        prev.borrow_mut().next = Some(node.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(node));
    }

    fn remove(node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().unwrap().upgrade().unwrap();
        let next = node.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
    }
}
