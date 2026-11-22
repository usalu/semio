//! 🫳️ Retained drag sessions, target acceptance and drop commits.

use super::*;

impl EventRouter {
    pub fn set_drag_payload(&mut self, node: NodeId, payload: DragPayload) {
        self.drag_payloads.insert(node, payload);
    }

    #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub fn clear_drag_payload(&mut self, node: NodeId) {
        self.drag_payloads.remove(&node);
    }

    #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub fn set_drop_accept(&mut self, node: NodeId, predicate: impl Fn(&DragPayload) -> bool + Send + Sync + 'static) {
        self.drop_accept.insert(node, Box::new(predicate));
    }

    pub fn drag_session(&self) -> Option<&DragSession> {
        self.drag.as_ref()
    }

    pub(super) fn leave_tree_row_reorder_target(&mut self) {
        if let Some(drag) = self.drag.as_mut().filter(|drag| drag.payload.contains_key(ui_contract::TREE_ROW_REORDER_MIME)) {
            drag.drop_target = None;
        }
    }

    /// 🫳️ Promotes a `Press` capture on a `drag_payloads`-registered node to `CaptureKind::Drag` once
    /// the pointer has moved past `DRAG_PROMOTE_THRESHOLD_SQ` from `press_origin`.
    pub(super) fn maybe_promote_to_drag(&mut self, x: f32, y: f32) {
        let Some((pointer_id, id, CaptureKind::Press)) = self.capture.target else { return };
        let Some(payload) = self.drag_payloads.get(&id).cloned() else { return };
        let Some((origin_x, origin_y)) = self.press_origin else { return };
        let palette = payload.contains_key(TREE_CATALOGUE_DRAG_MIME);
        if !(if palette { tree_palette_drag_begins(x - origin_x, y - origin_y) } else { (x - origin_x).powi(2) + (y - origin_y).powi(2) >= DRAG_PROMOTE_THRESHOLD_SQ }) {
            return;
        }
        self.capture.target = Some((pointer_id, id, CaptureKind::Drag));
        self.drag = Some(DragSession { source: id, payload, ghost: None, pointer_x: x, pointer_y: y, drop_target: None });
    }

    /// 🔄️ Live-updates the active `DragSession`'s pointer position and re-evaluates the drop target
    /// under it.
    pub(super) fn update_drag(&mut self, tree: &UiTree, root: NodeId, x: f32, y: f32) {
        if let Some(drag) = self.drag.as_mut() {
            drag.pointer_x = x;
            drag.pointer_y = y;
        }
        let target = self.hit_test(tree, root, x, y).and_then(|hit| self.nearest_accepting_drop_target(tree, hit));
        if let Some(drag) = self.drag.as_mut() {
            drag.drop_target = target;
        }
    }

    /// 🫳️ The key of the tree row or section `node` is — the key its owning tree's drop binding answers to.
    fn tree_drop_key(tree: &UiTree, node: NodeId) -> Option<&str> {
        tree.authored_tree_item(node).map(|item| item.id.as_str()).or_else(|| tree.authored_tree_section(node).map(|section| section.id.as_str()))
    }

    /// 🫳️ Whether `node` is a tree row or section whose owning tree binds a drop action — React's `dropBinding`.
    fn tree_node_drops(tree: &UiTree, node: NodeId) -> bool {
        Self::tree_drop_key(tree, node).zip(tree.document()).is_some_and(|(key, document)| semio_ui_retained_tree::reconcile::tree_drop_action(document, key).is_some())
    }

    /// 🫳️ Stamps the tree the drag in flight may drop on — React's `isDropReady` (`draggedIds.length > 0 && handleDrop`): the owning tree of a dragged row whose tree binds a drop action. Cleared when no drag is in flight.
    pub(super) fn sync_drop_ready(&self, tree: &mut UiTree) {
        let ready = self.drag.as_ref().and_then(|drag| {
            let owner = tree_row_owner(tree, drag.source)?;
            Self::tree_node_drops(tree, drag.source).then_some((owner, drag.source))
        });
        tree.set_drop_ready(ready);
    }

    /// 📚️ Whether a catalogue drop on `row` clicks it — React's `catalogueTreeDragController.handleDrop` when the tree binds no drop action: the tree's rows carry the catalogue MIME somewhere, the drop carries a non-blank payload under it, and the target is an item (its `onClick`: activation, else the domain pick).
    fn catalogue_row_click(tree: &UiTree, payload: &DragPayload, row: NodeId) -> bool {
        fn carries(items: &[UiTreeItemNode]) -> bool {
            items.iter().any(|item| item.drag_data.as_ref().is_some_and(|data| data.contains_key(TREE_CATALOGUE_DRAG_MIME)) || item.items.as_deref().is_some_and(carries))
        }
        payload.get(TREE_CATALOGUE_DRAG_MIME).map(String::as_str).filter(|encoded| tree_palette_payload(Some(*encoded))).is_some_and(|encoded| semio_framework_pack_json::parse(encoded, semio_framework_pack_json::JsonMemberPolicy::Replace).is_ok())
            && tree.authored_tree_item(row).is_some()
            && !Self::tree_node_drops(tree, row)
            && tree_row_owner(tree, row).and_then(|owner| super::tree_roving_and_keys::tree_spec(tree, owner)).is_some_and(|spec| spec.sections.iter().any(|section| carries(&section.items)))
    }

    /// 📏️ The row frame a drop on `row` is resolved against: the row's own band (its header when it is an open group), never the subtree behind it — the element React's `getBoundingClientRect` measures.
    pub fn tree_row_frame(&self, tree: &UiTree, row: NodeId) -> Option<Rect> {
        let rect = node_abs_rect(tree, row)?;
        let height = match tree.authored_tree_item(row) {
            Some(item) => semio_ui_wgpu_mounted_layout::retained_tree_row_metrics(tree, row, &self.tree_drag_metrics).for_item(item).row_height.min(rect.h),
            None => rect.h,
        };
        let y = if self.flow.block.is_reversed() { rect.y + rect.h - height } else { rect.y };
        Some(Rect::new(rect.x, y, rect.w, height))
    }

    /// 🎯️ Finds the nearest declared drop target whose predicate accepts the active drag.
    fn nearest_accepting_drop_target(&self, tree: &UiTree, from: NodeId) -> Option<NodeId> {
        let mime = ui_contract::TREE_ROW_REORDER_MIME;
        let reorder = self.drag.as_ref().is_some_and(|drag| drag.payload.contains_key(mime));
        let mut found = None;
        bubble(tree, from, |id| {
            let accepts_target = if reorder {
                self.drag.as_ref().is_some_and(|drag| drag.source != id) && tree.authored_tree_item(id).is_some_and(|item| item.drag_data.as_ref().is_some_and(|data| data.contains_key(mime))) && Self::tree_node_drops(tree, id)
            } else {
                tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::DROP_TARGET)) || Self::tree_node_drops(tree, id) || self.drag.as_ref().is_some_and(|drag| Self::catalogue_row_click(tree, &drag.payload, id))
            };
            if !accepts_target {
                return false;
            }
            let accepts = match self.drop_accept.get(&id) {
                Some(predicate) => self.drag.as_ref().is_some_and(|drag| predicate(&drag.payload)),
                None => true,
            };
            if accepts {
                found = Some(id);
                true
            } else {
                false
            }
        });
        found
    }

    pub(super) fn commit_drag(&mut self, tree: &mut UiTree, root: NodeId, active_id: NodeId, x: f32, y: f32) -> Vec<UiCommand> {
        self.tree_drag_handle_press = None;
        if let Some(node) = tree.node_mut(active_id) {
            node.flags.set(NodeFlags::ACTIVE, false);
        }
        tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
        self.update_drag(tree, root, x, y);
        let Some(drag) = self.drag.take() else { return Vec::new() };
        let window_id = self.window_id.clone();
        let mut out = Vec::new();
        match drag.drop_target {
            Some(target) if Self::catalogue_row_click(tree, &drag.payload, target) => {
                if let Some(owner) = tree_row_owner(tree, target) {
                    self.select_tree_row(tree, owner, target, None, TreeMerge::Replace, true, false, &mut out);
                }
            }
            Some(target) => out.push(UiCommand::DropCommitted { window_id, source: drag.source, target, y, payload: drag.payload }),
            None => out.push(UiCommand::DropCancelled { window_id, source: drag.source }),
        }
        out
    }
}
