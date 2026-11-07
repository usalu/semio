//! 🌲️ Owned AVL nodes with exact allocation extents and incremental free-list retirement.

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V> {
    entry: Option<(K, V)>,
    left: Link<K, V>,
    right: Link<K, V>,
    height: u8,
}

#[derive(Debug)]
pub(super) struct Ordered<K: Ord + Copy, V: Copy> {
    root: Link<K, V>,
    free: Link<K, V>,
    free_count: usize,
    allocated: usize,
}

impl<K: Ord + Copy, V: Copy> Ordered<K, V> {
    pub fn new() -> Self { Self { root: None, free: None, free_count: 0, allocated: 0 } }
    fn height(node: &Link<K, V>) -> u8 { node.as_ref().map_or(0, |node| node.height) }
    fn update(node: &mut Node<K, V>) { node.height = 1 + Self::height(&node.left).max(Self::height(&node.right)); }
    fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut right = node.right.take().expect("AVL right child");
        node.right = right.left.take();
        Self::update(&mut node);
        right.left = Some(node);
        Self::update(&mut right);
        right
    }
    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut left = node.left.take().expect("AVL left child");
        node.left = left.right.take();
        Self::update(&mut node);
        left.right = Some(node);
        Self::update(&mut left);
        left
    }
    fn balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        Self::update(&mut node);
        let delta = i16::from(Self::height(&node.left)) - i16::from(Self::height(&node.right));
        if delta > 1 {
            let left = node.left.as_ref().expect("AVL left branch");
            if Self::height(&left.left) < Self::height(&left.right) { node.left = Some(Self::rotate_left(node.left.take().expect("AVL left branch"))); }
            return Self::rotate_right(node);
        }
        if delta < -1 {
            let right = node.right.as_ref().expect("AVL right branch");
            if Self::height(&right.right) < Self::height(&right.left) { node.right = Some(Self::rotate_right(node.right.take().expect("AVL right branch"))); }
            return Self::rotate_left(node);
        }
        node
    }
    fn allocate(&mut self, key: K, value: V) -> Box<Node<K, V>> {
        let mut node = if let Some(mut node) = self.free.take() {
            self.free = node.left.take();
            self.free_count -= 1;
            node
        } else {
            self.allocated += 1;
            Box::new(Node { entry: None, left: None, right: None, height: 0 })
        };
        node.entry = Some((key, value));
        node.height = 1;
        node
    }
    fn release(&mut self, mut node: Box<Node<K, V>>) {
        node.entry = None;
        node.height = 0;
        node.left = self.free.take();
        node.right = None;
        self.free = Some(node);
        self.free_count += 1;
    }
    fn insert_at(&mut self, root: Link<K, V>, key: K, value: V) -> Box<Node<K, V>> {
        let Some(mut node) = root else { return self.allocate(key, value); };
        match key.cmp(&node.entry.expect("AVL entry").0) {
            std::cmp::Ordering::Less => node.left = Some(self.insert_at(node.left.take(), key, value)),
            std::cmp::Ordering::Greater => node.right = Some(self.insert_at(node.right.take(), key, value)),
            std::cmp::Ordering::Equal => { node.entry = Some((key, value)); return node; },
        }
        Self::balance(node)
    }
    pub fn insert(&mut self, key: K, value: V) { let root = self.root.take(); self.root = Some(self.insert_at(root, key, value)); }
    pub fn contains(&self, key: K) -> bool {
        let mut cursor = self.root.as_deref();
        while let Some(node) = cursor {
            match key.cmp(&node.entry.expect("AVL entry").0) {
                std::cmp::Ordering::Less => cursor = node.left.as_deref(),
                std::cmp::Ordering::Greater => cursor = node.right.as_deref(),
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }
    fn first_at(mut node: &Node<K, V>) -> (K, V) {
        while let Some(left) = node.left.as_deref() { node = left; }
        node.entry.expect("AVL first entry")
    }
    pub fn first(&self) -> Option<(K, V)> { self.root.as_deref().map(Self::first_at) }
    pub fn last(&self) -> Option<(K, V)> {
        let mut node = self.root.as_deref()?;
        while let Some(right) = node.right.as_deref() { node = right; }
        node.entry
    }
    pub fn after(&self, key: K) -> Option<(K, V)> {
        let mut cursor = self.root.as_deref();
        let mut found = None;
        while let Some(node) = cursor {
            let entry = node.entry.expect("AVL entry");
            if entry.0 > key { found = Some(entry); cursor = node.left.as_deref(); }
            else { cursor = node.right.as_deref(); }
        }
        found
    }
    fn remove_at(&mut self, root: Link<K, V>, key: K) -> Link<K, V> {
        let mut node = root?;
        match key.cmp(&node.entry.expect("AVL entry").0) {
            std::cmp::Ordering::Less => node.left = self.remove_at(node.left.take(), key),
            std::cmp::Ordering::Greater => node.right = self.remove_at(node.right.take(), key),
            std::cmp::Ordering::Equal => {
                if node.left.is_none() || node.right.is_none() {
                    let replacement = node.left.take().or_else(|| node.right.take());
                    self.release(node);
                    return replacement;
                }
                let successor = Self::first_at(node.right.as_deref().expect("AVL successor branch"));
                node.entry = Some(successor);
                node.right = self.remove_at(node.right.take(), successor.0);
            },
        }
        Some(Self::balance(node))
    }
    pub fn remove(&mut self, key: K) { let root = self.root.take(); self.root = self.remove_at(root, key); }
    pub fn pop_first(&mut self) -> Option<(K, V)> { let first = self.first()?; self.remove(first.0); Some(first) }
    pub fn next_release_bytes(&self) -> usize { if self.root.is_none() && self.free.is_some() { std::mem::size_of::<Node<K, V>>() } else { 0 } }
    pub fn retire_step(&mut self) -> bool {
        if self.pop_first().is_some() { return false; }
        if let Some(mut node) = self.free.take() {
            self.free = node.left.take();
            self.free_count -= 1;
            self.allocated -= 1;
            return false;
        }
        true
    }
    pub fn capacity_bytes(&self) -> usize { self.allocated * std::mem::size_of::<Node<K, V>>() }
    pub fn len(&self) -> usize { self.allocated - self.free_count }
    pub fn depth(&self) -> usize { usize::from(Self::height(&self.root)) }
    pub fn retirement_is_empty(&self) -> bool { self.allocated == 0 }
    pub fn allocation_demand(&self, additional: usize) -> Option<usize> { additional.saturating_sub(self.free_count).checked_mul(std::mem::size_of::<Node<K, V>>()) }
}

impl<K: Ord + Copy, V: Copy> Clone for Ordered<K, V> {
    fn clone(&self) -> Self {
        let mut output = Self::new();
        let mut next = self.first();
        while let Some((key, value)) = next { output.insert(key, value); next = self.after(key); }
        for _ in 0..self.free_count {
            let node = Box::new(Node { entry: None, left: None, right: None, height: 0 });
            output.allocated += 1;
            output.release(node);
        }
        output
    }
}

impl<K: Ord + Copy, V: Copy> Drop for Ordered<K, V> {
    fn drop(&mut self) { while !self.retire_step() {} }
}

#[cfg(test)]
#[path = "ordered.test.rs"]
mod tests;
