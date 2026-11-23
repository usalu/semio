//! 🔲️ Window frame geometry: one rectangle, glass chips inside it, content filling it. Rust twin of `window-frame-geometry/v1`, cased by `frame.cases.json`; see `frame.schema.json`.

use semio_ui_theme::geometry::Rect;

pub const WINDOW_FRAME_GEOMETRY_SCHEMA: &str = "window-frame-geometry/v1";
pub const WINDOW_FRAME_PATH_INSET: f32 = 1.0;
pub const WINDOW_FRAME_CHIP_EPSILON: f32 = 0.5;

/// 🔖️ The chip row a [`WindowFrameChipRegion`] is anchored to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowFrameDock {
    Top,
    Bottom,
}

/// 🏷️ Horizontal extent of one chip in frame-local coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowFrameChip {
    pub left: f32,
    pub right: f32,
}

impl WindowFrameChip {
    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }
}

/// 🏷️ One chip row: its depth and its normalized chips.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowFrameEdge {
    pub depth: f32,
    pub chips: Vec<WindowFrameChip>,
}

impl WindowFrameEdge {
    pub fn new(depth: f32, chips: Vec<WindowFrameChip>) -> Self {
        Self { depth, chips }
    }
}

/// 🪟️ A glass overlay inside the frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowFrameChipRegion {
    pub rect: Rect,
    pub dock: WindowFrameDock,
}

/// 📏️ Chip row depth per frame edge; document content starts below `top` and ends above `bottom`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowFrameSafeClearances {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// 🔲️ One axis-aligned frame at `bounds` with normalized chip rows; the outline and the content clip are `bounds`.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowFrame {
    pub bounds: Rect,
    pub top: WindowFrameEdge,
    pub bottom: WindowFrameEdge,
}

impl WindowFrame {
    pub fn new(bounds: Rect, top: WindowFrameEdge, bottom: WindowFrameEdge) -> Self {
        let width = bounds.w.max(0.0);
        let height = bounds.h.max(0.0);
        let top_depth = top.depth.max(0.0).min(height);
        let bottom_depth = bottom.depth.max(0.0).min((height - top_depth).max(0.0));
        Self {
            bounds: Rect::new(bounds.x, bounds.y, width, height),
            top: WindowFrameEdge::new(top_depth, Self::normalize_chips(top.chips, width)),
            bottom: WindowFrameEdge::new(bottom_depth, Self::normalize_chips(bottom.chips, width)),
        }
    }

    pub fn from_measured_edges(bounds: Rect, top_chips: Vec<WindowFrameChip>, bottom_chips: Vec<WindowFrameChip>, top_depth: f32, bottom_depth: f32) -> Self {
        Self::new(bounds, WindowFrameEdge::new(top_depth, top_chips), WindowFrameEdge::new(bottom_depth, bottom_chips))
    }

    pub fn from_measured_top(bounds: Rect, tabs_w: f32, controls_w: f32, depth: f32) -> Self {
        let tabs_right = tabs_w.max(0.0).min(bounds.w);
        let controls_left = (bounds.w - controls_w.max(0.0)).max(tabs_right).min(bounds.w);
        Self::from_measured_edges(bounds, vec![WindowFrameChip::new(0.0, tabs_right), WindowFrameChip::new(controls_left, bounds.w)], Vec::new(), depth, 0.0)
    }

    pub fn normalize_chips(chips: Vec<WindowFrameChip>, width: f32) -> Vec<WindowFrameChip> {
        let mut chips: Vec<_> = chips
            .into_iter()
            .filter(|chip| chip.left.is_finite() && chip.right.is_finite() && chip.right > chip.left)
            .map(|chip| WindowFrameChip::new(chip.left.clamp(0.0, width), chip.right.clamp(0.0, width)))
            .filter(|chip| chip.right - chip.left > WINDOW_FRAME_CHIP_EPSILON)
            .collect();
        chips.sort_by(|a, b| a.left.total_cmp(&b.left).then(a.right.total_cmp(&b.right)));
        let mut merged: Vec<WindowFrameChip> = Vec::new();
        for chip in chips {
            if let Some(last) = merged.last_mut() {
                if chip.left <= last.right + WINDOW_FRAME_CHIP_EPSILON {
                    last.right = last.right.max(chip.right);
                    continue;
                }
            }
            merged.push(chip);
        }
        merged
    }

    /// 🔲️ The content plane clip: the whole frame, chip rows included.
    pub fn content_clip_rect(&self) -> Rect {
        self.bounds
    }

    /// 🪟️ Chip rects inside the frame, top row first, left to right; rows at or below the epsilon depth have none.
    pub fn chip_regions(&self) -> Vec<WindowFrameChipRegion> {
        let b = self.bounds;
        let mut regions = Vec::with_capacity(self.top.chips.len() + self.bottom.chips.len());
        if self.top.depth > WINDOW_FRAME_CHIP_EPSILON {
            regions.extend(self.top.chips.iter().map(|chip| WindowFrameChipRegion { rect: Rect::new(b.x + chip.left, b.y, chip.right - chip.left, self.top.depth), dock: WindowFrameDock::Top }));
        }
        if self.bottom.depth > WINDOW_FRAME_CHIP_EPSILON {
            let y = b.y + b.h - self.bottom.depth;
            regions.extend(self.bottom.chips.iter().map(|chip| WindowFrameChipRegion { rect: Rect::new(b.x + chip.left, y, chip.right - chip.left, self.bottom.depth), dock: WindowFrameDock::Bottom }));
        }
        regions
    }

    pub fn safe_clearances(&self) -> WindowFrameSafeClearances {
        WindowFrameSafeClearances { top: self.top.depth, right: 0.0, bottom: self.bottom.depth, left: 0.0 }
    }

    /// 📐️ The frame without the chip rows: where document content rests when it must not scroll beneath the chips.
    pub fn clearance_rect(&self) -> Rect {
        Rect::new(self.bounds.x, self.bounds.y + self.top.depth, self.bounds.w, (self.bounds.h - self.top.depth - self.bottom.depth).max(0.0))
    }

    /// 🧭️ Inclusive point membership of the frame; chips are inside it, so there are no cutouts.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        let b = self.bounds;
        x.is_finite() && y.is_finite() && x >= b.x && x <= b.x + b.w && y >= b.y && y <= b.y + b.h
    }

    /// 🧭️ The chip under a point, if any; every other point of the frame is content.
    pub fn chip_at(&self, x: f32, y: f32) -> Option<WindowFrameChipRegion> {
        self.chip_regions().into_iter().find(|region| region.rect.contains(x, y))
    }

    /// 🖌️ The frame-local inset rectangle path, empty when the frame is not wider and taller than the epsilon.
    pub fn border_path(&self, inset: f32) -> String {
        let (w, h) = (self.bounds.w, self.bounds.h);
        if w <= WINDOW_FRAME_CHIP_EPSILON || h <= WINDOW_FRAME_CHIP_EPSILON {
            return String::new();
        }
        let i = inset.min(w / 2.0).min(h / 2.0);
        format!("M{i},{i} H{} V{} H{i} Z", w - i, h - i)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
