//! 🔦️ Focus and overlay api: the focus transitions and focus ring navigation and the overlay open, close and dismiss operations. See [`super`].

use super::*;

//#region 🔖️Outcome
impl Dispatcher {
    //#region 🔦️FocusApi
    /// 🎯️ Single choke point for every focus change (pointer-down onto a focusable node, Tab/Shift+Tab,
    /// an overlay closing out from under the focused node) — ported from `events.rs::FocusState::set_focus`'s
    /// role as the one place `EditState` lifecycle is owned: blurring drops the old element's edit
    /// buffer, focusing an [`DispatchFlags::EDITABLE`] element for the first time since its last blur
    /// seeds one from [`ListenerSet::value`] with the caret at the end.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn apply_focus_transition(&mut self, tree: &DispatchTree, target: Option<ElementId>) -> bool {
        if self.focus == target {
            return false;
        }
        if let Some(previous) = self.focus {
            self.edit_states.remove(&previous);
        }
        if let Some(next) = target {
            self.edit_states.entry(next).or_insert_with(|| {
                let value = tree.element_node(next).and_then(|id| tree.node(id)).and_then(|node| node.listeners.value.as_ref());
                let text = match value {
                    Some(UiValue::Text(text)) => text.to_string(),
                    _ => String::new(),
                };
                let caret = text.len();
                EditState { text, caret, anchor: caret, composition: None }
            });
        }
        self.focus = target;
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn clear_focus(&mut self, tree: &DispatchTree) -> bool {
        self.apply_focus_transition(tree, None)
    }

    /// 🎯️ Ported from `events.rs::FocusState::focus_next` verbatim, `NodeId` swapped for `ElementId`
    /// and the tab order rebuilt from [`collect_focusable`] scoped to `scope` (the topmost focus-trap
    /// overlay's root, or the tree root — see `Self::dispatch`'s `Tab` handling).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn focus_next(&mut self, tree: &DispatchTree, scope: FrameNodeId) -> bool {
        let mut order = Vec::with_capacity(64);
        collect_focusable(tree, scope, &mut order);
        if order.is_empty() {
            return self.apply_focus_transition(tree, None);
        }
        let next = match self.focus.and_then(|current| order.iter().position(|&candidate| candidate == current)) {
            Some(index) => (index + 1) % order.len(),
            None => 0,
        };
        self.apply_focus_transition(tree, Some(order[next]))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn focus_prev(&mut self, tree: &DispatchTree, scope: FrameNodeId) -> bool {
        let mut order = Vec::with_capacity(64);
        collect_focusable(tree, scope, &mut order);
        if order.is_empty() {
            return self.apply_focus_transition(tree, None);
        }
        let previous = match self.focus.and_then(|current| order.iter().position(|&candidate| candidate == current)) {
            Some(index) => (index + order.len() - 1) % order.len(),
            None => order.len() - 1,
        };
        self.apply_focus_transition(tree, Some(order[previous]))
    }
    //#endregion 🔦️FocusApi

    //#region 🪟️OverlayApi
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn open_overlay(&mut self, root: ElementId, kind: OverlayKind, anchor: OverlayAnchor) {
        let focus_trap = matches!(kind, OverlayKind::Dialog | OverlayKind::CommandPalette);
        self.overlays.open(OpenOverlay { root, kind, anchor, placement: kind.default_placement(), dismiss: kind.dismiss_policy(), focus_trap });
    }

    /// 🧹️ Ported from `events.rs::EventRouter::finish_close`: clears focus too if it was inside the
    /// closed overlay's subtree, so a dangling focus into a now-hidden subtree can't route key events
    /// nowhere useful.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn finish_close(&mut self, tree: &DispatchTree, overlay: OpenOverlay) {
        if let Some(focused) = self.focus {
            let inside = tree.element_node(focused).is_some_and(|focused_node| tree.element_node(overlay.root).is_some_and(|root_node| is_descendant(tree, focused_node, root_node)));
            if inside {
                self.clear_focus(tree);
            }
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn close_overlay(&mut self, tree: &DispatchTree, root: ElementId) -> bool {
        match self.overlays.close_root(root) {
            Some(overlay) => {
                self.finish_close(tree, overlay);
                true
            }
            None => false,
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn close_topmost_overlay(&mut self, tree: &DispatchTree) -> bool {
        match self.overlays.close_topmost() {
            Some(overlay) => {
                self.finish_close(tree, overlay);
                true
            }
            None => false,
        }
    }

    /// 🔽️ Generic replacement for `events.rs::toggle_select_popup`: activating a
    /// [`DispatchFlags::OVERLAY_TRIGGER`] node toggles an overlay whose root *and* anchor are that same
    /// node (a `Select`'s own synthesized item rows are already its children, so flagging the trigger
    /// itself `OVERLAY` gives the whole popup hit-test priority over its later-painted siblings).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn toggle_overlay_trigger(&mut self, tree: &DispatchTree, element: ElementId) {
        if let Some(overlay) = self.overlays.close_root(element) {
            self.finish_close(tree, overlay);
        } else {
            self.open_overlay(element, OverlayKind::SelectPopup, OverlayAnchor::Element(element));
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn dismiss_topmost_if_outside_press(&mut self, tree: &DispatchTree, x: f32, y: f32) -> bool {
        let Some(top) = self.overlays.topmost().copied() else { return false };
        if !top.dismiss.outside_press_swallow {
            return false;
        }
        let Some(root_node) = tree.element_node(top.root) else { return false };
        if hit_test_subtree(tree, root_node, x, y).is_some() {
            return false;
        }
        self.overlays.close_root(top.root);
        self.finish_close(tree, top);
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn maybe_dismiss_tooltip_on_hover_out(&mut self, tree: &DispatchTree, x: f32, y: f32) -> bool {
        let Some(top) = self.overlays.topmost().copied() else { return false };
        if top.kind != OverlayKind::Tooltip {
            return false;
        }
        let inside_overlay = tree.element_node(top.root).and_then(|id| node_bounds(tree, id)).is_some_and(|bounds| rect_contains(bounds, x, y));
        let inside_anchor = match top.anchor {
            OverlayAnchor::Element(element) => tree.element_node(element).and_then(|id| node_bounds(tree, id)).is_some_and(|bounds| rect_contains(bounds, x, y)),
            OverlayAnchor::Point { .. } => false,
        };
        if inside_overlay || inside_anchor {
            return false;
        }
        self.overlays.close_root(top.root);
        self.finish_close(tree, top);
        true
    }
    //#endregion 🪟️OverlayApi

}
//#endregion 🔖️Outcome
