//! 🪪️ Document identity and composite rows: the retained document binding, the interaction transfers, link clearing, child attach and detach and the composite row minting and retirement. See [`super`].

use super::*;

// #region tree
impl UiTree {
    //#region 🪪️DocumentIdentity
    /// 📄️ The arena node currently standing for `id`, if one is mounted.
    pub fn document_node(&self, id: UiNodeId) -> Option<NodeId> {
        self.document_nodes.binary_search_by_key(&id, |(document, _)| *document).ok().and_then(|index| self.document_nodes.get(index)).map(|(_, node)| *node)
    }

    pub fn document_id(&self, node: NodeId) -> Option<UiNodeId> {
        self.node(node)?.document_id
    }

    /// 🪢️ Binds `id` to `node`, replacing any previous binding for the same record.
    pub fn bind_document_node(&mut self, id: UiNodeId, node: NodeId) {
        let previous = match self.document_nodes.binary_search_by_key(&id, |(document, _)| *document) {
            Ok(index) => {
                if let Some(entry) = self.document_nodes.get_mut(index) {
                    let previous = entry.1;
                    entry.1 = node;
                    Some(previous)
                } else {
                    None
                }
            }
            Err(index) => {
                self.document_nodes.insert(index, (id, node));
                None
            }
        };
        if let Some(previous) = previous.filter(|previous| *previous != node) {
            if let Some(retained) = self.node_mut(previous) {
                retained.document_id = None;
            }
        }
        if let Some(retained) = self.node_mut(node) {
            retained.document_id = Some(id);
        }
    }

    /// 📒️ The ledger in `UiNodeId` order — the retirement sweep's own iteration order.
    pub fn document_bindings(&self) -> &[(UiNodeId, NodeId)] {
        &self.document_nodes
    }

    pub fn component_scene_host_is_mounted(&self, host_id: &str) -> bool {
        self.document_nodes.iter().any(|(_, node)| self.node(*node).is_some_and(|node| matches!(&node.spec.0, UiNode::ComponentScene(scene) if scene.host_id == host_id)))
    }

    /// 🎞️ Copies one retained interaction record shared by the same authored identity and widget
    /// kind. Candidate structure, specs, layout, paint, and document ownership stay independent.
    /// Returns `true` once `index` is beyond the fixed document ledger.
    pub fn transfer_interaction_record_from(&mut self, presented: &UiTree, index: usize) -> bool {
        let Some((document_id, target_id)) = self.document_nodes.get(index).copied() else {
            if let Some(root) = self.root {
                self.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
                self.mark_dirty(root, NodeFlags::DIRTY_PAINT);
            }
            return true;
        };
        let Some(source_id) = presented.document_node(document_id) else { return false };
        let _ = self.transfer_interaction_node_from(presented, source_id, target_id);
        false
    }

    pub fn transfer_composite_interaction_from(&mut self, presented: &UiTree) {
        for (source_owner, source_row) in &presented.composite_rows {
            let Some(document_id) = presented.document_id(*source_owner) else { continue };
            let Some(target_owner) = self.document_node(document_id) else { continue };
            let Some(source) = presented.node(*source_row) else { continue };
            let NodeKey::Explicit(key) = &source.key else { continue };
            let Some(target_row) = self.explicit_child(target_owner, key) else { continue };
            let _ = self.transfer_interaction_node_from(presented, *source_row, target_row);
        }
    }

    pub fn interaction_successor_from(&self, presented: &UiTree, source: NodeId) -> Option<NodeId> {
        let source_node = presented.node(source)?;
        let target = match presented.document_id(source) {
            Some(document_id) => self.document_node(document_id)?,
            None => {
                let source_owner = presented.surviving_composite_owner(source)?;
                let document_id = presented.document_id(source_owner)?;
                let target_owner = self.document_node(document_id)?;
                let NodeKey::Explicit(key) = &source_node.key else { return None };
                self.explicit_child(target_owner, key)?
            }
        };
        source_node.interaction_identity_matches(self.node(target)?).then_some(target)
    }

    fn transfer_interaction_node_from(&mut self, presented: &UiTree, source_id: NodeId, target_id: NodeId) -> bool {
        let Some(source) = presented.node(source_id) else { return false };
        let Some(target) = self.node_mut(target_id) else { return false };
        if !source.interaction_identity_matches(target) {
            return false;
        }
        target.state = source.state.clone();
        for flag in [NodeFlags::HOVERED, NodeFlags::ACTIVE, NodeFlags::FOCUSED, NodeFlags::FOCUS_VISIBLE, NodeFlags::OVERLAY] {
            target.flags.set(flag, source.flags.contains(flag));
        }
        target.flags.set(NodeFlags::DIRTY_LAYOUT, true);
        target.flags.set(NodeFlags::DIRTY_PAINT, true);
        true
    }

    /// ✂️ Drops the binding at `index` and returns the arena node it named.
    pub fn unbind_document_node_at(&mut self, index: usize) -> Option<NodeId> {
        if index >= self.document_nodes.len() {
            return None;
        }
        let node = self.document_nodes.remove(index).1;
        if let Some(retained) = self.node_mut(node) {
            retained.document_id = None;
        }
        Some(node)
    }

    /// 🔗️ Severs every tree link on `id` without touching the node itself — the first half of a
    /// relink pass, so a later `attach_child` can rebuild sibling order from the published document
    /// without any node losing its arena identity (and with it its `WidgetState`, its focused edit
    /// buffer and its interaction flags).
    pub fn clear_links(&mut self, id: NodeId) {
        if let Some(node) = self.arena.get_mut(id) {
            node.parent = None;
            node.first_child = None;
            node.last_child = None;
            node.prev_sibling = None;
            node.next_sibling = None;
        }
    }

    /// 📎️ Appends the already-inserted `child` as `parent`'s last child.
    pub fn attach_child(&mut self, parent: NodeId, child: NodeId) {
        let prev_last = self.arena.get(parent).and_then(|node| node.last_child);
        if let Some(node) = self.arena.get_mut(child) {
            node.parent = Some(parent);
            node.prev_sibling = prev_last;
            node.next_sibling = None;
        }
        if let Some(previous) = prev_last.and_then(|id| self.arena.get_mut(id)) {
            previous.next_sibling = Some(child);
        }
        if let Some(node) = self.arena.get_mut(parent) {
            if node.first_child.is_none() {
                node.first_child = Some(child);
            }
            node.last_child = Some(child);
        }
    }

    //#region 🔽️CompositeRows
    /// 🔽️ Mints one synthesized child under `owner` and ledgers it. `None` when the ledger is full,
    /// which a caller treats exactly like a row it could not mount — skipped, never a fault.
    pub fn mint_composite_row(&mut self, owner: NodeId, node: Node) -> Option<NodeId> {
        if self.composite_rows.len() >= UI_COMPOSITE_ROWS || !self.arena.contains(owner) {
            return None;
        }
        let row = self.arena.insert(node);
        self.attach_child(owner, row);
        self.composite_rows.push((owner, row));
        Some(row)
    }

    /// 👣️ Retires ONE synthesized row — unlinked from its owner and freed. `true` once the ledger is
    /// empty. Driven one row per step so a surface with a long popup open cannot spend a whole frame
    /// budget tearing it down.
    pub fn retire_composite_row_step(&mut self) -> bool {
        let Some((owner, row)) = self.composite_rows.pop() else { return true };
        self.detach_child(owner, row);
        self.remove(row);
        false
    }

    /// 🪦️ Retires ONE synthesized row belonging to `owner`. `true` once that owner has none left —
    /// what a closing popup drives, so its rows go with it instead of waiting for the next document
    /// reconcile to sweep the whole ledger.
    pub fn retire_composite_row_of_step(&mut self, owner: NodeId) -> bool {
        let Some(index) = self.composite_rows.iter().rposition(|(candidate, _)| *candidate == owner) else { return true };
        let (_, row) = self.composite_rows.remove(index);
        self.detach_child(owner, row);
        self.remove(row);
        false
    }

    /// 🔢️ How many synthesized rows this tree holds — what a law reads to prove the working set is
    /// bounded and that a closed popup left nothing behind.
    pub fn composite_row_count(&self) -> usize {
        self.composite_rows.len()
    }

    /// 📂️ Whether `row` is one synthesized option of a currently open Select popup. These rows
    /// remain children of their trigger so capture and keyed reconciliation share one owner, but
    /// their paint and pointer priority is overlay priority rather than ordinary child priority.
    pub fn is_open_select_popup_row(&self, row: NodeId) -> bool {
        let Some(owner) = self.composite_rows.iter().find_map(|(owner, candidate)| (*candidate == row).then_some(*owner)) else { return false };
        self.node(owner).is_some_and(|node| matches!(&node.spec.0, UiNode::Select(_)) && node.state.open && node.state.select_popup.is_some())
    }

    /// 🔒️ The still-published Select owner of a synthesized `row`. A captured option uses this to
    /// preserve its owner's rows across a host document refresh; a removed or retyped owner answers
    /// `None`, so the reconcile retires the row and the gesture terminates without dispatch.
    pub fn surviving_composite_owner(&self, row: NodeId) -> Option<NodeId> {
        let owner = self.composite_rows.iter().find_map(|(owner, candidate)| (*candidate == row).then_some(*owner))?;
        let document_id = self.document_id(owner)?;
        self.document.as_ref().and_then(|document| document.record(document_id)).filter(|record| matches!(&record.component, ui_contract::Component::Select(_))).map(|_| owner)
    }

    /// ❔️ Whether `owner` already has its synthesized rows this generation.
    pub fn composite_rows_of(&self, owner: NodeId) -> usize {
        self.composite_rows.iter().filter(|(candidate, _)| *candidate == owner).count()
    }

    /// 🗑️ Retires one synthesized row outside `preserved_owner`. `true` means only that captured
    /// owner's rows remain, or the ledger is empty.
    pub fn retire_composite_row_except_step(&mut self, preserved_owner: Option<NodeId>) -> bool {
        let Some(index) = self.composite_rows.iter().rposition(|(owner, _)| Some(*owner) != preserved_owner) else { return true };
        let (owner, row) = self.composite_rows.remove(index);
        self.detach_child(owner, row);
        self.remove(row);
        false
    }

    /// ⛓️ Restores a preserved owner's synthesized child chain after the document relink cleared
    /// its tree links. The fixed ledger order is the popup's existing row order.
    pub fn reattach_composite_rows(&mut self, owner: NodeId) {
        for index in 0..self.composite_rows.len() {
            let (candidate, row) = self.composite_rows[index];
            if candidate == owner && self.contains(row) {
                self.attach_child(owner, row);
            }
        }
    }

    /// 🔌️ Unlinks `child` from `parent`'s sibling chain without touching `child`'s own subtree — the
    /// inverse of [`UiTree::attach_child`], so a retired composite row leaves the chain it was
    /// appended to exactly as it found it.
    fn detach_child(&mut self, parent: NodeId, child: NodeId) {
        let (prev, next) = match self.arena.get(child) {
            Some(node) if node.parent == Some(parent) => (node.prev_sibling, node.next_sibling),
            _ => return,
        };
        match prev {
            Some(prev) => {
                if let Some(node) = self.arena.get_mut(prev) {
                    node.next_sibling = next;
                }
            }
            None => {
                if let Some(node) = self.arena.get_mut(parent) {
                    node.first_child = next;
                }
            }
        }
        match next {
            Some(next) => {
                if let Some(node) = self.arena.get_mut(next) {
                    node.prev_sibling = prev;
                }
            }
            None => {
                if let Some(node) = self.arena.get_mut(parent) {
                    node.last_child = prev;
                }
            }
        }
        if let Some(node) = self.arena.get_mut(child) {
            node.parent = None;
            node.prev_sibling = None;
            node.next_sibling = None;
        }
    }
    //#endregion 🔽️CompositeRows

    /// 🍃️ Inserts `node` unparented and unlinked — the reconcile links it in a later step.
    pub fn insert_detached(&mut self, node: Node) -> NodeId {
        self.arena.insert(node)
    }

    /// 🧹️ Frees one already-unlinked slot. Unlike [`UiTree::remove`] this never recurses, because a
    /// relink pass has already cleared the node's children.
    pub fn remove_detached(&mut self, id: NodeId) {
        if self.root == Some(id) {
            self.root = None;
        }
        drop(self.arena.remove(id));
    }

    /// 🪜️ Retires one document identity binding per call, freeing the arena slot it named — the
    /// retirement half of the ledger, driven by `Ui::close_document_step` so a surface that drops its
    /// document does not keep its records' arena nodes alive. `true` once the ledger is empty.
    pub fn close_document_binding_step(&mut self, retire_scene: &mut impl FnMut(UiNodeId, NodeId, &Node) -> bool) -> bool {
        if !self.retire_composite_row_step() {
            return false;
        }
        let Some((document_id, node)) = self.document_nodes.last().copied() else { return true };
        let Some(retained) = self.arena.get(node) else {
            self.document_nodes.pop();
            return false;
        };
        if matches!(&retained.spec.0, UiNode::ComponentScene(_)) && !retire_scene(document_id, node, retained) {
            return false;
        }
        self.document_nodes.pop();
        self.clear_links(node);
        self.remove_detached(node);
        false
    }
    /// 📪️ Releases the empty tree's remaining indices after document retirement.
    pub fn close_storage_step(&mut self, retire_scene: &mut impl FnMut(NodeId, &Node) -> bool) -> bool {
        if self.document.is_some() || !self.document_nodes.is_empty() || !self.composite_rows.is_empty() {
            return false;
        }
        if self.overlay_origins.pop().is_some() {
            return false;
        }
        if self.overlay_origins.capacity() > 0 {
            self.overlay_origins = Vec::new();
            return false;
        }
        if self.document_nodes.capacity() > 0 {
            self.document_nodes = Vec::new();
            return false;
        }
        if self.composite_rows.capacity() > 0 {
            self.composite_rows = Vec::new();
            return false;
        }
        if let Some((id, node)) = self.arena.last() {
            if matches!(&node.spec.0, UiNode::ComponentScene(_)) && !retire_scene(id, node) {
                return false;
            }
            self.clear_links(id);
            self.remove_detached(id);
            return false;
        }
        self.arena.close_vacant_step()
    }
    //#endregion 🪪️DocumentIdentity
}
// #endregion tree
