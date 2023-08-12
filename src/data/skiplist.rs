use std::rc::Rc;

const MAX_HEIGHT: usize = 12;
const BRANCH_FACTOR: usize = 4;

pub struct SkipList {}

pub struct SkipListNode<K, V> {
    key: K,
    value: V,
    height: usize,
    next: Rc<SkipListNode<K, V>>,
}