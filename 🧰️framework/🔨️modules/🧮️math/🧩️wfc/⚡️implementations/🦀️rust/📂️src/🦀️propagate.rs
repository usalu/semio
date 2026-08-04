//! 🌀️ The propagation queue shared by every propagation engine. The engines themselves (AC-3 in
//! this phase, AC-4/watched-support in a later phase) live in sibling `prop_*.rs` modules.

use crate::ids::NodeId;

// #region 🔖️Queue
/// 🌀️ A FIFO node queue with membership-bit dedup — pushing an already-queued node is a no-op, so
/// a node dirtied twice before being processed is still visited exactly once per drain.
#[derive(Clone, Debug)]
pub(crate) struct PropQueue {
    queue: std::collections::VecDeque<u32>,
    in_queue: Vec<bool>,
}

impl PropQueue {
    pub fn new(node_count: usize) -> Self {
        Self { queue: std::collections::VecDeque::new(), in_queue: vec![false; node_count] }
    }

    pub fn push(&mut self, n: NodeId) {
        let idx = n.index();
        if !self.in_queue[idx] {
            self.in_queue[idx] = true;
            self.queue.push_back(n.get());
        }
    }

    pub fn pop(&mut self) -> Option<NodeId> {
        let raw = self.queue.pop_front()?;
        self.in_queue[raw as usize] = false;
        Some(NodeId(raw))
    }

    #[allow(dead_code)] // queue-introspection API exercised by the step/resume API added in a later phase
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
        self.in_queue.iter_mut().for_each(|b| *b = false);
    }

    pub fn push_all(&mut self, node_count: usize) {
        self.clear();
        for i in 0..node_count {
            self.push(NodeId::from_index(i));
        }
    }
}
// #endregion 🔖️Queue

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_dedups_and_fifo_orders() {
        let mut q = PropQueue::new(4);
        q.push(NodeId(1));
        q.push(NodeId(2));
        q.push(NodeId(1)); // dedup
        assert_eq!(q.pop(), Some(NodeId(1)));
        assert_eq!(q.pop(), Some(NodeId(2)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn popped_node_can_be_repushed() {
        let mut q = PropQueue::new(2);
        q.push(NodeId(0));
        q.pop();
        q.push(NodeId(0));
        assert_eq!(q.pop(), Some(NodeId(0)));
    }

    #[test]
    fn clear_resets_membership() {
        let mut q = PropQueue::new(2);
        q.push(NodeId(0));
        q.clear();
        assert!(q.is_empty());
        q.push(NodeId(0));
        assert_eq!(q.pop(), Some(NodeId(0)));
    }

    #[test]
    fn push_all_enqueues_every_node_once() {
        let mut q = PropQueue::new(3);
        q.push_all(3);
        let mut seen = Vec::new();
        while let Some(n) = q.pop() {
            seen.push(n);
        }
        assert_eq!(seen, vec![NodeId(0), NodeId(1), NodeId(2)]);
    }
}
// #endregion 🔖️Tests
