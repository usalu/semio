//! 🧲️ The tree drop rule — the Rust twin of `🧲️drag-drop/🟦️.tsx` (`resolveTreeDropPosition`, `isTreeReorderDragEvent`, `TreeReorderDropPreview`) and of the accept rule in `🟦️.tsx` (`handleDragOverItem`). One rule, two hosts: both read the neutral vectors `🧪️tests/🧫️fixtures/🧲️drag-drop/🔣️.json`. Pure: a host passes the row frame and the pointer, this answers where the drop lands and which preview shape (insert line or nest box) it draws.
//! @see 🎫️ 26/09/17 WGPU-RENDERER-REACT-PARITY · 📓️opus55-wave3-tree-drag.md

/// 📍️ The fraction of a row's height, from each edge, that means "before" or "after" — React's `0.25` / `0.75` cut.
pub const TREE_DROP_EDGE_FRACTION: f32 = 0.25;

/// 📏️ The insert line's thickness in px — React's `h-0.5`.
pub const TREE_DROP_LINE_THICKNESS: f32 = 2.0;

/// 🔲️ The nest box's border width in px — React's `border-2`.
pub const TREE_DROP_BOX_BORDER: f32 = 2.0;

/// 🎨️ The nest box's border alpha — React's `border-primary/80`.
pub const TREE_DROP_BOX_BORDER_ALPHA: f32 = 0.8;

/// 🎨️ The nest box's fill alpha — React's `bg-primary/10`.
pub const TREE_DROP_BOX_FILL_ALPHA: f32 = 0.1;

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

/// 👁️ The shape one preview draws.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TreeDropShape {
    /// ➖️ A 2 px insert line.
    Line([f32; 4]),
    /// 🔲️ The nest box over the whole row.
    Box([f32; 4]),
}

/// 👁️ The preview a drag shows over the row `frame` (`[x, y, w, h]`), `None` when the row refuses it. A positional row drag previews only a row that carries the row-reorder MIME (`target_accepts`), is not itself being dragged and is hit on an edge; a plain tree drag previews every position, nest included; a palette drag previews nothing.
pub fn tree_drop_preview(kind: TreeDragKind, position: TreeDrop, target_accepts: bool, target_is_source: bool, frame: [f32; 4]) -> Option<TreeDropShape> {
    let [x, y, w, h] = frame;
    let accepted = match kind {
        TreeDragKind::RowReorder => target_accepts && !target_is_source && position != TreeDrop::Inside,
        TreeDragKind::Plain => true,
        TreeDragKind::Other => false,
    };
    accepted.then(|| match position {
        TreeDrop::Before => TreeDropShape::Line([x, y, w, TREE_DROP_LINE_THICKNESS]),
        TreeDrop::After => TreeDropShape::Line([x, y + h - TREE_DROP_LINE_THICKNESS, w, TREE_DROP_LINE_THICKNESS]),
        TreeDrop::Inside => TreeDropShape::Box(frame),
    })
}

/// 🔲️ The four strips of a nest box's `border-2` inside `frame`: top, bottom, left, right.
pub fn tree_drop_box_border(frame: [f32; 4]) -> [[f32; 4]; 4] {
    let [x, y, w, h] = frame;
    let b = TREE_DROP_BOX_BORDER.min(w * 0.5).min(h * 0.5);
    [[x, y, w, b], [x, y + h - b, w, b], [x, y + b, b, (h - 2.0 * b).max(0.0)], [x + w - b, y + b, b, (h - 2.0 * b).max(0.0)]]
}

#[cfg(test)]
#[path = "../🧪️tests/🧲️drag-drop/🦀️.rs"]
mod tests;
