//! 🔗️ Tree structure edits: child insertion and removal with the sibling links, dirty marking and the children iterator. See [`super`].

use super::*;

// #region tree
impl UiTree {

    /// 🔗️ Inserts `node` as the last child of `parent` (or as a root if `parent` is `None` and no
    /// root exists yet), threading the sibling links.
    pub fn insert_child(&mut self, parent: Option<NodeId>, mut node: Node) -> NodeId {
        node.parent = parent;
        let id = self.arena.insert(node);
        match parent {
            Some(parent_id) => {
                let prev_last = self.arena.get(parent_id).and_then(|p| p.last_child);
                if let Some(prev_last_id) = prev_last {
                    if let Some(prev_last_node) = self.arena.get_mut(prev_last_id) {
                        prev_last_node.next_sibling = Some(id);
                    }
                }
                if let Some(child) = self.arena.get_mut(id) {
                    child.prev_sibling = prev_last;
                }
                if let Some(parent_node) = self.arena.get_mut(parent_id) {
                    if parent_node.first_child.is_none() {
                        parent_node.first_child = Some(id);
                    }
                    parent_node.last_child = Some(id);
                }
            }
            None => {
                if self.root.is_none() {
                    self.root = Some(id);
                }
            }
        }
        id
    }

    /// 🧹️ Detaches `id` from its parent/siblings and recursively removes its subtree, freeing every
    /// arena slot involved.
    pub fn remove(&mut self, id: NodeId) {
        let Some(node) = self.arena.get(id) else { return };
        let (parent, prev_sibling, next_sibling) = (node.parent, node.prev_sibling, node.next_sibling);
        let children: Vec<NodeId> = self.children(id).collect();
        for child in children {
            self.remove(child);
        }
        match prev_sibling {
            Some(prev_id) => {
                if let Some(prev) = self.arena.get_mut(prev_id) {
                    prev.next_sibling = next_sibling;
                }
            }
            None => {
                if let Some(parent_id) = parent {
                    if let Some(parent_node) = self.arena.get_mut(parent_id) {
                        parent_node.first_child = next_sibling;
                    }
                }
            }
        }
        match next_sibling {
            Some(next_id) => {
                if let Some(next) = self.arena.get_mut(next_id) {
                    next.prev_sibling = prev_sibling;
                }
            }
            None => {
                if let Some(parent_id) = parent {
                    if let Some(parent_node) = self.arena.get_mut(parent_id) {
                        parent_node.last_child = prev_sibling;
                    }
                }
            }
        }
        if self.root == Some(id) {
            self.root = None;
        }
        self.arena.remove(id);
    }

    /// 🚨️ Sets `flags` on `id` (setting `DIRTY_LAYOUT` implies `DIRTY_PAINT`, since layout changes
    /// always require a repaint), then bubbles `SUBTREE_DIRTY` to the root that owns scheduling.
    /// An accepted generation clears the root before descendant flags, so an already-marked
    /// intermediate ancestor is terminal only while that root still carries a layout obligation.
    pub fn mark_dirty(&mut self, id: NodeId, flags: NodeFlags) {
        let mut flags = flags;
        if flags.contains(NodeFlags::DIRTY_LAYOUT) {
            flags.set(NodeFlags::DIRTY_PAINT, true);
        }
        let root_layout_dirty = self.root.and_then(|root| self.arena.get(root)).is_some_and(|root| root.flags.contains(NodeFlags::DIRTY_LAYOUT) || root.flags.contains(NodeFlags::SUBTREE_DIRTY));
        let parent = match self.arena.get_mut(id) {
            Some(node) => {
                node.flags.set(flags, true);
                node.parent
            }
            None => return,
        };
        let mut cursor = parent;
        while let Some(ancestor_id) = cursor {
            let Some(ancestor) = self.arena.get_mut(ancestor_id) else { break };
            if root_layout_dirty && ancestor.flags.contains(NodeFlags::SUBTREE_DIRTY) {
                break;
            }
            ancestor.flags.set(NodeFlags::SUBTREE_DIRTY, true);
            cursor = ancestor.parent;
        }
    }

    /// 🚶️ Iterates the direct children of `id` in tree order via the first-child/next-sibling links.
    pub fn children(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let mut next = self.arena.get(id).and_then(|n| n.first_child);
        std::iter::from_fn(move || {
            let current = next?;
            next = self.arena.get(current).and_then(|n| n.next_sibling);
            Some(current)
        })
    }
}
// #endregion tree
