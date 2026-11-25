//! 📍️ Drop places of a drag, matching `places.ts` and the language-neutral `places.cases.json`: which place lies under the pointer, where the travelling copy lies, when a press becomes a drag, which slot an insert takes and which band of a tree row a drop lands in.
//!
//! A place is a rectangle inside its frame; every place a dragged element can be dropped on is tinted `DROP_TINT_AVAILABLE_PERCENT`, the one under the pointer `DROP_TINT_ACTIVE_PERCENT`. See `places.schema.json`.

/// 🎨️ Share in percent of the active colour that tints every place the element can be dropped on.
pub const DROP_TINT_AVAILABLE_PERCENT: f32 = 10.0;

/// 🎨️ Share in percent of the active colour that tints the place under the pointer.
pub const DROP_TINT_ACTIVE_PERCENT: f32 = 32.0;

/// 👻️ Offset in px of the travelling copy from the pointer, so the copy never hides the place under the pointer.
pub const DRAG_GHOST_OFFSET: [f32; 2] = [8.0, 10.0];

/// 📏️ Pointer travel in px before a press becomes a drag.
pub const DRAG_START_THRESHOLD_PX: f32 = 4.0;

/// 🔲️ An axis aligned rectangle given by its edges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl DropRect {
    /// 🔲️ The rectangle of an `[x, y, w, h]` frame.
    pub fn from_frame([x, y, w, h]: [f32; 4]) -> Self {
        Self { left: x, top: y, right: x + w, bottom: y + h }
    }

    /// 🔲️ The `[x, y, w, h]` frame of the rectangle.
    pub fn frame(self) -> [f32; 4] {
        [self.left, self.top, self.right - self.left, self.bottom - self.top]
    }

    fn area(self) -> f32 {
        (self.right - self.left) * (self.bottom - self.top)
    }

    fn contains(self, x: f32, y: f32) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }
}

/// 🎯️ One place the dragged element can be dropped on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropPlace<'id> {
    pub id: &'id str,
    pub rect: DropRect,
}

/// 🚦️ A place is `Available` while the element travels and `Active` while the pointer is over it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropPlaceState {
    Available,
    Active,
}

impl DropPlaceState {
    /// 🎨️ The share in percent of the active colour that tints a place in this state.
    pub const fn tint_percent(self) -> f32 {
        match self {
            Self::Available => DROP_TINT_AVAILABLE_PERCENT,
            Self::Active => DROP_TINT_ACTIVE_PERCENT,
        }
    }

    /// 🎨️ The alpha factor of the active colour that tints a place in this state.
    pub const fn tint_alpha(self) -> f32 {
        self.tint_percent() / 100.0
    }
}

/// 🎯️ The place under the point: the smallest one that contains it, of equal ones the later (the one painted on top).
pub fn drop_place_at<'id>(places: &[DropPlace<'id>], x: f32, y: f32) -> Option<&'id str> {
    let mut best: Option<&DropPlace<'id>> = None;
    for place in places {
        if place.rect.contains(x, y) && best.is_none_or(|current| place.rect.area() <= current.rect.area()) {
            best = Some(place);
        }
    }
    best.map(|place| place.id)
}

/// 👻️ Where the travelling copy lies for a pointer at `(x, y)`.
pub fn drag_ghost_position(x: f32, y: f32) -> [f32; 2] {
    [x + DRAG_GHOST_OFFSET[0], y + DRAG_GHOST_OFFSET[1]]
}

/// 🚀️ Whether the pointer has travelled far enough from where it was pressed for the press to become a drag.
pub fn drag_starts(from: [f32; 2], to: [f32; 2]) -> bool {
    (to[0] - from[0]).hypot(to[1] - from[1]) >= DRAG_START_THRESHOLD_PX
}

/// ⎋ Whether the key abandons the drag.
pub fn drag_cancel_key(key: &str) -> bool {
    key == "Escape"
}

/// ↔️ The rectangle inside a row where an element dropped at `index` among `items` would lie: the first half of the item that follows it, at the end the second half of the last one, an empty row is the whole slot.
pub fn insert_slot(row: DropRect, items: &[DropRect], index: i64) -> DropRect {
    let Some(last) = items.last() else { return row };
    let at = index.clamp(0, items.len() as i64) as usize;
    match items.get(at) {
        Some(next) => DropRect { left: next.left, top: row.top, right: (next.left + next.right) / 2.0, bottom: row.bottom },
        None => DropRect { left: (last.left + last.right) / 2.0, top: row.top, right: last.right, bottom: row.bottom },
    }
}

/// 🌳️ Where a drop lands in a tree row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeBand {
    Before,
    After,
    Inside,
}

/// 🌳️ The rectangle inside a tree row where a drop lands: the upper quarter before it, the lower quarter after it, the whole row inside it.
pub fn tree_drop_band(row: DropRect, band: TreeBand) -> DropRect {
    let quarter = (row.bottom - row.top) / 4.0;
    match band {
        TreeBand::Before => DropRect { bottom: row.top + quarter, ..row },
        TreeBand::After => DropRect { top: row.bottom - quarter, ..row },
        TreeBand::Inside => row,
    }
}

#[cfg(test)]
#[path = "places.test.rs"]
mod tests;
