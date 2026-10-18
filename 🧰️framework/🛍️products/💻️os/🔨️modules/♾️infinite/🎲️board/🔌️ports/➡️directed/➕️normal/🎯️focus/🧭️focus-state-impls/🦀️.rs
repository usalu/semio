//! 🧭️ Focus state impls of the board focus: further methods of the focus state. See [`super`].

use super::*;

impl BoardFocusState {
    pub fn close_interaction_step(&mut self) -> bool {
        match &mut self.interaction {
            Interaction::None | Interaction::Pan { .. } => {
                self.interaction = Interaction::None;
                true
            }
            Interaction::DragNodes { primary_id, start_positions, proximity_pair, gesture, .. } => {
                if let Some((id, _)) = start_positions.pop_first() {
                    drop(id);
                    return false;
                }
                if let Some(id) = gesture.select.as_mut().and_then(|(ids, _)| ids.pop()).or_else(|| gesture.restore.as_mut().and_then(BTreeSet::pop_first)) {
                    drop(id);
                    return false;
                }
                drop(std::mem::take(gesture));
                if let Some((left, right)) = proximity_pair.take() {
                    if right.is_empty() {
                        drop(left);
                    } else {
                        drop(right);
                        *proximity_pair = Some((left, String::new()));
                    }
                    return false;
                }
                drop(std::mem::take(primary_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::SelectionPending { initial_ids, .. } => {
                if let Some(id) = initial_ids.pop_first() {
                    drop(id);
                    return false;
                }
                self.interaction = Interaction::None;
                true
            }
            Interaction::Selection { initial_ids, points, screen_points, .. } => {
                if initial_ids.pop_first().is_some() || points.pop().is_some() || screen_points.pop().is_some() {
                    return false;
                }
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkAtSourceHandle { source_id, .. } => {
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkDragSnap { source_id, target_id, .. } => {
                if let Some(target) = target_id.take() {
                    drop(target);
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkTargetNode { source_id, target_node_id } => {
                if !target_node_id.is_empty() {
                    drop(std::mem::take(target_node_id));
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } => {
                if let Some(id) = compatible_node_ids.pop().or_else(|| ring_handle_ids.pop()).or_else(|| ring_node_id.take()) {
                    drop(id);
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
        }
    }
}

impl BoardFocusState {
    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
        self.selection_options.method = method.into();
        self.selection_options.mode = normalize_selection_mode(mode);
        self.selection_options.select_nodes = select_nodes;
        self.selection_options.select_edges = select_edges;
        self.selection_options.select_handles = select_handles;
    }

    pub(super) fn hovered_style_kind(&self, id: &str, domain: &str, element_kind: &str) -> Option<BoardElementStyleKind> {
        if self.is_preselect_active() {
            return None;
        }
        if self.selection.contains(id) {
            return None;
        }
        if self.hovered_id.as_deref() == Some(id) {
            return Some(BoardElementStyleKind::Hovered);
        }
        if self.hovered_id.is_none() {
            if let Some((hover_domain, hover_kind)) = self.hovered_kind.as_ref() {
                if hover_domain == domain && hover_kind == element_kind {
                    return Some(BoardElementStyleKind::Hovered);
                }
            }
        }
        None
    }

    pub(super) fn highlighted_style_kind(&self, id: &str) -> Option<BoardElementStyleKind> {
        if self.is_preselect_active() || self.selection.contains(id) {
            return None;
        }
        if self.hovered_id.as_deref() == Some(id) {
            return None;
        }
        if self.highlighted_ids.contains(id) {
            return Some(BoardElementStyleKind::Highlighted);
        }
        None
    }

    pub fn is_preselect_active(&self) -> bool {
        self.is_preselecting() || !self.preselect.is_empty()
    }

    /// 🎨️ During area-select: preselect → Selected; anchor∖preselect → Highlighted; idle selection → Selected.
    pub(super) fn resolve_interaction_style_kind(&self, id: &str) -> BoardElementStyleKind {
        if self.is_preselect_active() {
            if self.preselect.contains(id) {
                return BoardElementStyleKind::Selected;
            }
            if self.selection.contains(id) {
                return BoardElementStyleKind::Highlighted;
            }
            return BoardElementStyleKind::Neutral;
        }
        if self.selection.contains(id) {
            return BoardElementStyleKind::Selected;
        }
        if self.highlighted_ids.contains(id) {
            return BoardElementStyleKind::Highlighted;
        }
        BoardElementStyleKind::Neutral
    }

    pub(super) fn resolve_edge_style_kind(&self, e: &EdgeData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = explicit_style_kind(e.style.as_deref()) {
            return locked_style_dim(kind, e.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(e.id.as_str(), "edge", e.edge_kind.as_str()) {
                    return locked_style_dim(kind, e.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(e.id.as_str()) {
                    return locked_style_dim(kind, e.locked);
                }
                self.resolve_interaction_style_kind(e.id.as_str())
            }
        };
        locked_style_dim(kind, e.locked)
    }

    pub(crate) fn resolve_wire_style_kind(&self, w: &WireData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = explicit_style_kind(w.style.as_deref()) {
            return locked_style_dim(kind, w.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(w.id.as_str(), "wire", w.wire_kind.as_str()) {
                    return locked_style_dim(kind, w.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(w.id.as_str()) {
                    return locked_style_dim(kind, w.locked);
                }
                self.resolve_interaction_style_kind(w.id.as_str())
            }
        };
        locked_style_dim(kind, w.locked)
    }
}
