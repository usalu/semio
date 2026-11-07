//! 🧲️ Shared cell-model docking placements and corner geometry used by signals and frame consumers.

use crate::geometry::{Pos, Rect};

/// ⚓️ A tab joins a group or creates a sibling on one of its four edges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockPlacement { Tabs, Corner(WindowStackCorner), Left, Right, Top, Bottom }

impl DockPlacement {
    /// 🗺️ Resolves corner tab pads before the regular UI body split on its dominant axis.
    pub fn at(area: Rect, pos: Pos) -> Option<Self> {
        if !area.contains(pos) || area.width < 3 || area.height < 3 { return None; }
        for corner in [WindowStackCorner::TopLeft, WindowStackCorner::TopRight, WindowStackCorner::BottomLeft, WindowStackCorner::BottomRight] {
            if Self::Corner(corner).preview(area).contains(pos) { return Some(Self::Corner(corner)); }
        }
        let x = 2 * i32::from(pos.x - area.x) - i32::from(area.width);
        let y = 2 * i32::from(pos.y - area.y) - i32::from(area.height);
        Some(if x.abs() >= y.abs() { if x < 0 { Self::Left } else { Self::Right } } else if y < 0 { Self::Top } else { Self::Bottom })
    }

    /// 🔲️ The proposed area, including the shared splitter cell.
    pub fn preview(self, area: Rect) -> Rect {
        let width = area.width.saturating_sub(1).div_ceil(2);
        let height = area.height.saturating_sub(1).div_ceil(2);
        match self {
            Self::Left => Rect::new(area.x, area.y, width + (area.width > 0) as u16, area.height),
            Self::Right => Rect::new(area.x + width, area.y, area.width.saturating_sub(width), area.height),
            Self::Top => Rect::new(area.x, area.y, area.width, height + (area.height > 0) as u16),
            Self::Bottom => Rect::new(area.x, area.y + height, area.width, area.height.saturating_sub(height)),
            Self::Tabs => area,
            Self::Corner(corner) => {
                let width = (if corner.is_left() { area.width.div_ceil(2) } else { area.width / 2 }).min(24);
                let height = area.height.min(2);
                Rect::new(if corner.is_left() { area.x } else { area.x + area.width - width }, if corner.is_top() { area.y } else { area.y + area.height - height }, width, height)
            }
        }
    }
}

/// 🧭️ Corner of a window stack where a tab chip docks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowStackCorner {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl WindowStackCorner {
    pub fn is_top(self) -> bool {
        matches!(self, Self::TopLeft | Self::TopRight)
    }

    pub fn is_left(self) -> bool {
        matches!(self, Self::TopLeft | Self::BottomLeft)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
