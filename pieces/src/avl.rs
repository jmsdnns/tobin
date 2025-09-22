#![allow(dead_code, unused)]

use std::cmp::{Ordering, max};

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
    V: Ord + Clone,
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
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + max(Self::height(&self.left), Self::height(&self.right));
    }

    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) - Self::height(&self.right)
    }

    fn rotate_left(mut root: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
        let mut new_root = root.right.take().unwrap();
        root.right = new_root.left.take();
        root.update_height();
        new_root.left = Some(root);
        new_root.update_height();
        new_root
    }

    fn rotate_right(mut root: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
        let mut new_root = root.left.take().unwrap();
        root.left = new_root.right.take();
        root.update_height();
        new_root.right = Some(root);
        new_root.update_height();
        new_root
    }

    fn rebalance(mut node: Box<AvlNode<K, V>>) -> Box<AvlNode<K, V>> {
        let balance = node.balance_factor();

        // heavy on the left
        if balance > 1 {
            if let Some(left) = &node.left {
                if left.balance_factor() < 0 {
                    // left right
                    node.left = Some(Self::rotate_left(node.left.take().unwrap()));
                }
            }
            // left left
            return Self::rotate_right(node);
        }

        // heavy on the right
        if balance < -1 {
            if let Some(right) = &node.right {
                // right left
                if right.balance_factor() > 0 {
                    node.right = Some(Self::rotate_right(node.right.take().unwrap()));
                }
            }
            // right right
            return Self::rotate_left(node);
        }

        node
    }

    fn insert(node: Option<Box<AvlNode<K, V>>>, key: K, value: V) -> Box<AvlNode<K, V>> {
        let mut node = match node {
            Some(n) => n,
            None => return Box::new(AvlNode::new(key, value)),
        };

        match key.cmp(&node.key) {
            Ordering::Less => {
                node.left = Some(Self::insert(node.left.take(), key, value));
            }
            Ordering::Greater => {
                node.right = Some(Self::insert(node.right.take(), key, value));
            }
            Ordering::Equal => {
                node.value = value; // overwrite existing key
                return node; // rebalancing is unnecessary
            }
        }

        node.update_height();
        Self::rebalance(node)
    }

    fn get(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Less => {
                if let Some(left) = &self.left {
                    left.get(key)
                } else {
                    None
                }
            }
            Ordering::Greater => {
                if let Some(right) = &self.right {
                    right.get(key)
                } else {
                    None
                }
            }
            Ordering::Equal => Some(&self.value),
        }
    }

    fn collect_all(&self, result: &mut Vec<(K, V)>) {
        if let Some(left) = &self.left {
            left.collect_all(result);
        }

        result.push((self.key.clone(), self.value.clone()));

        if let Some(right) = &self.right {
            right.collect_all(result);
        }
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
    V: Ord + Clone,
{
    pub fn new() -> Self {
        AvlTree {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let key_exists = self.get(&key).is_some();
        self.root = Some(AvlNode::insert(self.root.take(), key, value));
        if !key_exists {
            self.size += 1;
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match &self.root {
            Some(root) => root.get(key),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> i32 {
        AvlNode::height(&self.root)
    }

    pub fn flush(&self) -> Vec<(K, V)> {
        let mut result = Vec::new();
        if let Some(root) = &self.root {
            root.collect_all(&mut result);
        }
        result
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }

    pub fn should_flush(&self, max_size: usize) -> bool {
        self.size >= max_size
    }
}

impl<K, V> Default for AvlTree<K, V>
where
    K: Ord + Clone,
    V: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree: AvlTree<i32, String> = AvlTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn test_single_insert_and_get() {
        let mut tree = AvlTree::new();
        tree.insert(5, "five".to_string());

        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.height(), 1);
        assert_eq!(tree.get(&5), Some(&"five".to_string()));
        assert_eq!(tree.get(&3), None);
    }

    #[test]
    fn test_multiple_inserts() {
        let mut tree = AvlTree::new();
        tree.insert(5, "five".to_string());
        tree.insert(3, "three".to_string());
        tree.insert(7, "seven".to_string());
        tree.insert(1, "one".to_string());
        tree.insert(9, "nine".to_string());

        assert_eq!(tree.len(), 5);
        assert_eq!(tree.get(&1), Some(&"one".to_string()));
        assert_eq!(tree.get(&3), Some(&"three".to_string()));
        assert_eq!(tree.get(&5), Some(&"five".to_string()));
        assert_eq!(tree.get(&7), Some(&"seven".to_string()));
        assert_eq!(tree.get(&9), Some(&"nine".to_string()));
        assert_eq!(tree.get(&2), None);
    }

    #[test]
    fn test_duplicate_key_insert() {
        let mut tree = AvlTree::new();
        tree.insert(5, "five".to_string());
        tree.insert(5, "five_updated".to_string());

        assert_eq!(tree.len(), 1); // still be 1 since we overwrote
        assert_eq!(tree.get(&5), Some(&"five_updated".to_string()));
    }

    #[test]
    fn test_flush() {
        let mut tree = AvlTree::new();
        tree.insert(5, "five".to_string());
        tree.insert(3, "three".to_string());
        tree.insert(7, "seven".to_string());
        tree.insert(1, "one".to_string());

        let flushed = tree.flush();

        // should be in sorted order by key
        assert_eq!(flushed.len(), 4);
        assert_eq!(flushed[0], (1, "one".to_string()));
        assert_eq!(flushed[1], (3, "three".to_string()));
        assert_eq!(flushed[2], (5, "five".to_string()));
        assert_eq!(flushed[3], (7, "seven".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut tree = AvlTree::new();
        tree.insert(5, "five".to_string());
        tree.insert(3, "three".to_string());
        tree.insert(7, "seven".to_string());

        assert_eq!(tree.len(), 3);
        assert!(!tree.is_empty());

        tree.clear();

        assert_eq!(tree.len(), 0);
        assert!(tree.is_empty());
        assert_eq!(tree.height(), 0);
        assert_eq!(tree.get(&5), None);
    }

    #[test]
    fn test_should_flush() {
        let mut tree = AvlTree::new();

        assert!(!tree.should_flush(5));

        tree.insert(1, "one".to_string());
        tree.insert(2, "two".to_string());
        tree.insert(3, "three".to_string());

        assert!(!tree.should_flush(5));

        tree.insert(4, "four".to_string());
        tree.insert(5, "five".to_string());

        assert!(tree.should_flush(5));
        assert!(tree.should_flush(3));
    }

    #[test]
    fn test_height_balancing() {
        let mut tree = AvlTree::new();

        // insert in ascending order to test balancing
        for i in 1..=7 {
            tree.insert(i, format!("value_{}", i));
        }

        // tree should maintain balance with height at log(n)
        assert!(
            tree.height() <= 4,
            "Tree height {} is too large for {} nodes",
            tree.height(),
            tree.len()
        );
    }

    #[test]
    fn test_empty_flush() {
        let tree: AvlTree<i32, String> = AvlTree::new();
        let flushed = tree.flush();
        assert!(flushed.is_empty());
    }

    #[test]
    fn test_large_tree() {
        let mut tree = AvlTree::new();

        // 100 elements!
        for i in 0..100 {
            tree.insert(i, format!("value_{}", i));
        }

        assert_eq!(tree.len(), 100);

        // all elements are retrievable
        for i in 0..100 {
            assert_eq!(tree.get(&i), Some(&format!("value_{}", i)));
        }

        // non-existent elements return None
        assert_eq!(tree.get(&200), None);

        // height should be log(n) for 100 nodes, eg around 6-7
        assert!(
            tree.height() <= 8,
            "Tree height {} is too large for 100 nodes",
            tree.height()
        );
    }

    #[test]
    fn test_reverse_order_insertion() {
        let mut tree = AvlTree::new();

        // descending order
        for i in (1..=10).rev() {
            tree.insert(i, format!("value_{}", i));
        }

        // all elements are retrievable
        for i in 1..=10 {
            assert_eq!(tree.get(&i), Some(&format!("value_{}", i)));
        }

        // flush returns sorted order
        let flushed = tree.flush();
        for (idx, (key, _)) in flushed.iter().enumerate() {
            assert_eq!(*key, (idx + 1) as i32);
        }
    }
}
