use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use bytes::Bytes;
use rand::random;

const MAX_HEIGHT: usize = 12;
const BRANCH_FACTOR: usize = 4;

pub struct SkipList {
    head: Rc<RefCell<SkipListNode>>,
    current_height: usize,
}

pub struct SkipListNode {
    key: Bytes,
    value: Bytes,
    next: Vec<Option<Rc<RefCell<SkipListNode>>>>,
}

impl SkipListNode {
    pub fn new(key: Bytes, value: Bytes) -> Self {
        SkipListNode {
            key,
            value,
            next: vec![None; MAX_HEIGHT],
        }
    }
}

impl SkipList {
    pub fn new() -> Self {
        let head = SkipListNode {
            key: Bytes::new(),
            value: Bytes::new(),
            next: vec![None; MAX_HEIGHT],
        };
        SkipList {
            head: Rc::new(RefCell::new(head)),
            current_height: 1,
        }
    }

    fn less_than_eq(key: &Bytes, opt_node: &Option<Rc<RefCell<SkipListNode>>>) -> bool {
        if opt_node.is_none() {
            return true;
        }
        let node = opt_node.as_ref().unwrap();
        let order = key.cmp(&node.borrow().key);
        return match order {
            Ordering::Greater => false,
            _ => true
        };
    }

    fn find_greater_or_eq(&self, key: &Bytes) -> (Option<Rc<RefCell<SkipListNode>>>, Vec<Rc<RefCell<SkipListNode>>>) {
        let mut prev_nodes_by_levels = vec![self.head.clone(); MAX_HEIGHT];
        let mut level = self.current_height - 1;
        let mut current_node = self.head.clone();
        loop {
            let next_node_opt = current_node.borrow().next[level].clone();
            if Self::less_than_eq(key, &next_node_opt) {
                prev_nodes_by_levels[level] = current_node.clone();
                if level == 0 {
                    return (next_node_opt, prev_nodes_by_levels);
                }
                level -= 1;
                continue;
            }
            current_node = next_node_opt.as_ref().unwrap().clone();
        }
    }

    fn calculate_random_height(&self) -> usize {
        let mut height = 1;
        loop {
            if height >= MAX_HEIGHT || random::<usize>() % BRANCH_FACTOR != 0 {
                break;
            }
            height += 1;
        }
        height
    }

    pub fn insert(&mut self, key: Bytes, value: Bytes) {
        let (found_next, mut prev_nodes_by_levels) = self.find_greater_or_eq(&key);
        if found_next.is_some() && key.cmp(&found_next.as_ref().unwrap().borrow().key) == Ordering::Equal {
            panic!("Same keys are not allowed here");
        }
        let height = self.calculate_random_height();
        // Expand the scope of levels
        if height > self.current_height {
            let iter = prev_nodes_by_levels.iter_mut().skip(self.current_height).take(height);
            for n in iter {
                *n = self.head.clone()
            }
            self.current_height = height
        }
        let new_node = Rc::new(
            RefCell::new(SkipListNode::new(key, value))
        );
        for h in 1..=height {
            let level = h - 1;
            new_node.borrow_mut().next[level] = prev_nodes_by_levels[level].borrow().next[level].clone();
            prev_nodes_by_levels[level].borrow_mut().next[level] = Some(new_node.clone())
        }
    }

    pub fn contain(&self, key: &Bytes) -> bool {
        let got = self.get(key);
        got.is_some()
    }

    pub fn get(&self, key: &Bytes) -> Option<Bytes> {
        let (found_next, _) = self.find_greater_or_eq(key);
        if found_next.is_some() {
            let node = &found_next.as_ref().unwrap().borrow();
            if key.cmp(&node.key) == Ordering::Equal {
                return Some(node.value.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::data::skiplist::SkipList;

    #[test]
    fn it_works() {
        let mut skip_list = SkipList::new();
        let key = Bytes::from("hello");
        let value = Bytes::from("world");
        skip_list.insert(key.clone(), value.clone());
        let got = skip_list.get(&key);
        assert_eq!(value.clone(), got.unwrap());
        assert_eq!(skip_list.contain(&key), true);
    }
}