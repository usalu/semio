//! 🪟️ Overlays, scroll and edit state: the overlay anchors and stack with their placement, the nearest scrollable ancestor and the edit state with its caret helpers. See [`super`].

use super::*;

//#region 🔖️Propagation
//#region 🪟️Overlay
/// ⚓️ Ported from `events.rs::OverlayAnchor`, `NodeId` swapped for the stable [`ElementId`] (an
/// overlay anchor must survive across the frame boundary that opens it, unlike a [`FrameNodeId`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayAnchor {
    Element(ElementId),
    Point { x: f32, y: f32 },
}

/// 🪟️ Ported from `events.rs::OpenOverlay`, `root`/`anchor` swapped to [`ElementId`] for the same
/// cross-frame-survival reason as [`OverlayAnchor`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenOverlay {
    pub root: ElementId,
    pub kind: OverlayKind,
    pub anchor: OverlayAnchor,
    pub placement: OverlayPlacement,
    pub dismiss: DismissPolicy,
    pub focus_trap: bool,
}

/// 🥞️ Ported from `events.rs::OverlayStack` verbatim (z-order: last = topmost = hit-tested first).
#[derive(Default, Clone, Debug)]
pub(super) struct OverlayStack {
    pub(super) open: Vec<OpenOverlay>,
}

impl OverlayStack {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn open(&mut self, overlay: OpenOverlay) {
        if self.open.len() < 16 {
            self.open.push(overlay);
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn topmost(&self) -> Option<&OpenOverlay> {
        self.open.last()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn close_root(&mut self, root: ElementId) -> Option<OpenOverlay> {
        let position = self.open.iter().position(|overlay| overlay.root == root)?;
        Some(self.open.remove(position))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn close_topmost(&mut self) -> Option<OpenOverlay> {
        self.open.pop()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn topmost_focus_trap_root(&self) -> Option<ElementId> {
        self.open.iter().rev().find(|overlay| overlay.focus_trap).map(|overlay| overlay.root)
    }
}

/// 📐️ React's positioner over this module's own anchor vocabulary — a forwarding shim over
/// `ui_contract::resolve_overlay_placement`, with `OverlayAnchor::Element` resolved through
/// [`DispatchTree::element_node`]. `flow` is React's `rtl` argument (`useFlow().inline`).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn resolve_overlay_placement(tree: &DispatchTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement, flow: FlowInline) -> (f32, f32) {
    let resolved = resolve_overlay_placement_side(tree, anchor, content_size, viewport, placement, flow);
    (resolved.x, resolved.y)
}

/// 📐️ [`resolve_overlay_placement`] plus the side a collision flip settled on, for a caller that
/// also wants React's `transformOrigin`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn resolve_overlay_placement_side(tree: &DispatchTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement, flow: FlowInline) -> ResolvedOverlayPlacement {
    let anchor_rect = match anchor {
        OverlayAnchor::Element(element) => tree.element_node(element).and_then(|id| node_bounds(tree, id)).map_or(OverlayRect::default(), |bounds| OverlayRect::new(bounds.x, bounds.y, bounds.w, bounds.h)),
        OverlayAnchor::Point { x, y } => OverlayRect::point(x, y),
    };
    ui_contract::resolve_overlay_placement(anchor_rect, content_size, viewport, placement, flow)
}
//#endregion 🪟️Overlay

//#region 🖱️Scroll
/// 🖱️ Ported from `events.rs::nearest_scrollable_ancestor` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn nearest_scrollable_ancestor(tree: &DispatchTree, from: FrameNodeId) -> Option<FrameNodeId> {
    let mut found = None;
    bubble(tree, from, |id| {
        if tree.node(id).is_some_and(|node| node.flags.contains(DispatchFlags::SCROLLABLE)) {
            found = Some(id);
            true
        } else {
            false
        }
    });
    found
}
//#endregion 🖱️Scroll

//#region ✍️EditRouting
/// ✍️ Ported from `events.rs`'s `tree::EditState` shape, minus `scroll_x` — caret-into-view horizontal
/// scroll is a paint-time concern belonging to whichever `Element` renders the caret (it consults
/// [`Dispatcher::edit_state`] and its own retained scroll position), never dispatch's own job now that
/// there is no paint step in this module.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EditState {
    pub text: String,
    pub caret: usize,
    pub anchor: usize,
    pub composition: Option<String>,
}

/// ✍️ Ported from `events.rs::prev_char_boundary` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn prev_char_boundary(text: &str, index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    let mut candidate = index - 1;
    while candidate > 0 && !text.is_char_boundary(candidate) {
        candidate -= 1;
    }
    candidate
}

/// ✍️ Ported from `events.rs::next_char_boundary` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn next_char_boundary(text: &str, index: usize) -> usize {
    if index >= text.len() {
        return text.len();
    }
    let mut candidate = index + 1;
    while candidate < text.len() && !text.is_char_boundary(candidate) {
        candidate += 1;
    }
    candidate
}

/// ↔️ Ported from `events.rs::selection_bounds` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
    (anchor.min(caret), anchor.max(caret))
}

/// ✍️ Ported from `events.rs::insert_at_caret` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn insert_at_caret(edit: &mut EditState, text: &str) {
    let (start, end) = selection_bounds(edit.anchor, edit.caret);
    edit.text.replace_range(start..end, text);
    let caret = start + text.len();
    edit.caret = caret;
    edit.anchor = caret;
}
//#endregion ✍️EditRouting
//#endregion 🔖️Propagation
