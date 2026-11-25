//! 🧲️ The tree drop rule — the Rust twin of `🧲️drag-drop/🟦️.tsx` (`resolveTreeDropPosition`, `isTreeReorderDragEvent`, `TreeReorderDropPreview`) and of the accept rule in `🟦️.tsx` (`handleDragOverItem`). One rule, two hosts: both read the neutral vectors `🧪️tests/🧫️fixtures/🧲️drag-drop/🔣️.json` and the shared band cases `🔨️modules/🫳️pointer-drag/places.cases.json`. Pure: a host passes the row frame and the pointer, this answers where the drop lands and which band of the row is the active drop place (a rectangle inside the row, tinted at `DROP_TINT_ACTIVE_PERCENT`).
//! @see 🎫️ 26/09/17 WGPU-RENDERER-REACT-PARITY · 📓️opus55-wave3-tree-drag.md

use super::drag_places::{tree_drop_band, DropRect, TreeBand};

/// 📍️ The fraction of a row's height, from each edge, that means "before" or "after" — React's `0.25` / `0.75` cut.
pub const TREE_DROP_EDGE_FRACTION: f32 = 0.25;

/// 📚️ The MIME a catalogue (palette) row carries its encoded payload under — React's `CATALOGUE_DRAG_MIME`.
pub const TREE_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

/// 🖱️ The squared pointer travel in px² a palette gesture needs before it becomes a drag — React's `deltaX² + deltaY² < 36`.
pub const TREE_PALETTE_DRAG_THRESHOLD_SQ: f32 = 36.0;

/// 📚️ Whether a row's drag data starts a palette drag: the catalogue MIME holds a non-blank payload — `readEncodedDragPayload` (`payload?.trim() ? payload : undefined`).
pub fn tree_palette_payload(payload: Option<&str>) -> bool {
    payload.is_some_and(|payload| !payload.trim().is_empty())
}

/// 🖱️ Whether a palette press that moved `(dx, dy)` has become a drag.
pub fn tree_palette_drag_begins(dx: f32, dy: f32) -> bool {
    dx * dx + dy * dy >= TREE_PALETTE_DRAG_THRESHOLD_SQ
}

/// 📍️ Where a drop lands relative to the row under the pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeDrop {
    Before,
    After,
    Inside,
}

impl TreeDrop {
    /// 🏷️ The wire spelling a drop action carries as `dropPosition`; `None` for the nest drop, which a document tree never emits.
    pub fn wire(self) -> Option<&'static str> {
        match self {
            Self::Before => Some("before"),
            Self::After => Some("after"),
            Self::Inside => None,
        }
    }
}

/// 📍️ Resolves whether a drop at pointer `y` lands before, after or inside the row spanning `top .. top + height` — `resolveTreeDropPosition`, edge for edge.
pub fn tree_drop_position(top: f32, height: f32, y: f32) -> TreeDrop {
    let offset = y - top;
    if offset < height * TREE_DROP_EDGE_FRACTION {
        TreeDrop::Before
    } else if offset > height * (1.0 - TREE_DROP_EDGE_FRACTION) {
        TreeDrop::After
    } else {
        TreeDrop::Inside
    }
}

/// 🫳️ What a drag in flight carries, as the drop preview sees it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeDragKind {
    /// 📍️ A positional row drag: the payload names the row-reorder MIME.
    RowReorder,
    /// 🔀️ Only the tree's own default MIME — `isTreeReorderDragEvent`.
    Plain,
    /// 📚️ Any other payload (a palette or catalogue drag): no preview.
    Other,
}

/// 🫳️ The `isTreeReorderDragEvent` / `handleDragOverItem` classification of a payload's MIME kinds: `row_reorder` is whether the row-reorder MIME is present; `default_mime` whether the tree's own default MIME is; `foreign` whether any other `application/*` kind is.
pub fn tree_drag_kind(row_reorder: bool, default_mime: bool, foreign: bool) -> TreeDragKind {
    if row_reorder {
        TreeDragKind::RowReorder
    } else if default_mime && !foreign {
        TreeDragKind::Plain
    } else {
        TreeDragKind::Other
    }
}

/// 🌳️ The band of the row a drop at `position` lands in, in the vocabulary of the shared drop places.
pub const fn tree_band(position: TreeDrop) -> TreeBand {
    match position {
        TreeDrop::Before => TreeBand::Before,
        TreeDrop::After => TreeBand::After,
        TreeDrop::Inside => TreeBand::Inside,
    }
}

/// 👁️ The active drop place a drag shows over the row `frame` (`[x, y, w, h]`): the band of the row the drop lands in, `None` when the row refuses it. A positional row drag previews only a row that carries the row-reorder MIME (`target_accepts`), is not itself being dragged and is hit on an edge; a plain tree drag previews every position, nest included; a palette drag previews nothing.
pub fn tree_drop_preview(kind: TreeDragKind, position: TreeDrop, target_accepts: bool, target_is_source: bool, frame: [f32; 4]) -> Option<[f32; 4]> {
    let accepted = match kind {
        TreeDragKind::RowReorder => target_accepts && !target_is_source && position != TreeDrop::Inside,
        TreeDragKind::Plain => true,
        TreeDragKind::Other => false,
    };
    accepted.then(|| tree_drop_band(DropRect::from_frame(frame), tree_band(position)).frame())
}

#[cfg(test)]
#[path = "../🧪️tests/🧲️drag-drop/🦀️.rs"]
mod tests;
