//! 🫳️ Retained drag sessions, target acceptance and drop commits.

use super::*;

impl EventRouter {
    //#region 🔖️DragDropApi
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
        if (x - origin_x).powi(2) + (y - origin_y).powi(2) < DRAG_PROMOTE_THRESHOLD_SQ {
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

    /// 🎯️ Finds the nearest declared drop target whose predicate accepts the active drag.
    fn nearest_accepting_drop_target(&self, tree: &UiTree, from: NodeId) -> Option<NodeId> {
        let mime = ui_contract::TREE_ROW_REORDER_MIME;
        let reorder = self.drag.as_ref().is_some_and(|drag| drag.payload.contains_key(mime));
        let mut found = None;
        bubble(tree, from, |id| {
            let accepts_target = if reorder {
                tree.authored_tree_item(id).zip(tree.document()).is_some_and(|(item, document)| {
                    item.drag_data.as_ref().is_some_and(|data| data.contains_key(mime)) && semio_ui_retained_tree::reconcile::tree_item_drop_action(document, &item.id).is_some()
                })
            } else {
                tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::DROP_TARGET))
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
    //#endregion 🔖️DragDropApi

    pub(super) fn commit_drag(&mut self, tree: &mut UiTree, root: NodeId, active_id: NodeId, x: f32, y: f32) -> Option<UiCommand> {
        self.tree_drag_handle_press = None;
        if let Some(node) = tree.node_mut(active_id) {
            node.flags.set(NodeFlags::ACTIVE, false);
        }
        tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
        self.update_drag(tree, root, x, y);
        let drag = self.drag.take()?;
        let window_id = self.window_id.clone();
        Some(match drag.drop_target {
            Some(target) => UiCommand::DropCommitted { window_id, source: drag.source, target, y, payload: drag.payload },
            None => UiCommand::DropCancelled { window_id, source: drag.source },
        })
    }
}
