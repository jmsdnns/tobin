#![allow(dead_code, unused)]

#[derive(Debug, Clone)]
struct AvlNode<K, V> {
    key: K,
    value: V,
    height: i32,
    left: Option<Box<AvlNode<K, V>>>,
    right: Option<Box<AvlNode<K, V>>>,
}

impl<K, V> AvlNode<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn new(key: K, value: V) -> Self {
        AvlNode {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: &Option<Box<AvlNode<K, V>>>) -> i32 {
        todo!()
    }

    fn update_height(&mut self) {
        todo!()
    }

    fn balance_factor(&self) -> i32 {
        todo!()
    }

    fn rotate_left(mut root: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
        todo!()
    }

    fn rotate_right(mut root: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
        todo!()
    }

    fn insert(node: Option<Box<AvlNode<K, V>>>, key: K, value: V) -> Box<AvlNode<K, V>> {
        todo!()
    }

    fn rebalance(mut node: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
        todo!()
    }

    fn collect_all(&self, result: &mut Vec<(K, V)>) {
        todo!()
    }

    fn get(&self, key: &K) -> Option<&V> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct AvlTree<K, V> {
    root: Option<Box<AvlNode<K, V>>>,
    size: usize,
}

impl<K, V> AvlTree<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        AvlTree {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        todo!()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn height(&self) -> i32 {
        todo!()
    }

    pub fn flush(&self) -> Vec<(K, V)> {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn should_flush(&self, max_size: usize) -> bool {
        todo!()
    }
}

impl<K, V> Default for AvlTree<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn default() -> Self {
        todo!()
    }
}
