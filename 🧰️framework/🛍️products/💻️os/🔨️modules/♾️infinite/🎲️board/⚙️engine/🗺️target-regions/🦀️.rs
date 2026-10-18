//! 🗺️ Target regions of the board engine: the region constants, region data, bounds, grips, grip hit tests, grip drags and the scalar snap. See [`super`].

use super::*;

// #region 🔖️TargetRegions
pub const REGION_GRIP_PX: f64 = ui_styling::metrics::board::REGION_GRIP_PX;

pub const REGION_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::REGION_HIT_TOLERANCE_PX;

pub const REGION_MIN_EXTENT_WORLD: f64 = ui_styling::metrics::board::REGION_MIN_EXTENT_WORLD;

pub const REGION_LABEL_INSET_PX: f64 = ui_styling::metrics::board::REGION_LABEL_INSET_PX;

/// 🎯️ One axis-aligned board rectangle constraining fill placement — the engine's own copy of the
/// document's `targetRegions` row. `width`/`height` are measured from `x`/`y`; a negative extent is
/// normalized by every reader through [`region_bounds`], so the corner a brush started from never
/// reaches a hit test or a paint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegionData {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub label: Option<String>,
    pub hidden: bool,
    pub locked: bool,
    pub selected: bool,
}

/// 📐️ Normalized `[min_x, min_y, max_x, max_y]` of a rectangle stated as corner-plus-extent.
pub fn region_bounds(x: f64, y: f64, width: f64, height: f64) -> [f64; 4] {
    let (min_x, max_x) = if width < 0.0 { (x + width, x) } else { (x, x + width) };
    let (min_y, max_y) = if height < 0.0 { (y + height, y) } else { (y, y + height) };
    [min_x, min_y, max_x, max_y]
}

/// 🤏️ Which part of a region rectangle a pointer grabbed. Corners outrank edges and edges outrank
/// the body, so a press in the overlap of two grips always resolves to the more specific one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionGrip {
    Body,
    West,
    East,
    North,
    South,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl RegionGrip {
    /// 🏷️ Wire name — the probe reads it back out of `data-board-interaction-json`.
    pub fn name(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::West => "west",
            Self::East => "east",
            Self::North => "north",
            Self::South => "south",
            Self::NorthWest => "northWest",
            Self::NorthEast => "northEast",
            Self::SouthWest => "southWest",
            Self::SouthEast => "southEast",
        }
    }

    /// 📐️ Whether this grip moves the minimum/maximum edge on each axis: `(min_x, max_x, min_y, max_y)`.
    /// `Body` moves all four, which is exactly a translation.
    pub fn edges(self) -> (bool, bool, bool, bool) {
        match self {
            Self::Body => (true, true, true, true),
            Self::West => (true, false, false, false),
            Self::East => (false, true, false, false),
            Self::North => (false, false, true, false),
            Self::South => (false, false, false, true),
            Self::NorthWest => (true, false, true, false),
            Self::NorthEast => (false, true, true, false),
            Self::SouthWest => (true, false, false, true),
            Self::SouthEast => (false, true, false, true),
        }
    }
}

/// 🎯️ The grip `point` lands on, or `None` when it misses the rectangle and its grip band entirely.
/// The band is a constant SCREEN width, so grips stay grabbable at every zoom.
pub fn region_grip_at(bounds: [f64; 4], zoom: f64, point: Point) -> Option<RegionGrip> {
    let zoom = if zoom.is_finite() && zoom > 1e-9 { zoom } else { 1.0 };
    let tolerance = REGION_HIT_TOLERANCE_PX / zoom;
    let [min_x, min_y, max_x, max_y] = bounds;
    if point.x < min_x - tolerance || point.x > max_x + tolerance || point.y < min_y - tolerance || point.y > max_y + tolerance {
        return None;
    }
    let west = (point.x - min_x).abs() <= tolerance;
    let east = (point.x - max_x).abs() <= tolerance;
    let north = (point.y - min_y).abs() <= tolerance;
    let south = (point.y - max_y).abs() <= tolerance;
    Some(match (west, east, north, south) {
        (true, _, true, _) => RegionGrip::NorthWest,
        (_, true, true, _) => RegionGrip::NorthEast,
        (true, _, _, true) => RegionGrip::SouthWest,
        (_, true, _, true) => RegionGrip::SouthEast,
        (true, _, _, _) => RegionGrip::West,
        (_, true, _, _) => RegionGrip::East,
        (_, _, true, _) => RegionGrip::North,
        (_, _, _, true) => RegionGrip::South,
        _ => RegionGrip::Body,
    })
}

/// 🚚️ Applies a world drag delta to the edges this grip owns and answers the new normalized bounds.
/// Neither extent may collapse: an edge pushed past its opposite is clamped at [`REGION_MIN_EXTENT_WORLD`],
/// so a resize can never mint a zero- or negative-area region the fill rule would read as "nothing fits".
pub fn region_grip_drag(bounds: [f64; 4], grip: RegionGrip, dx: f64, dy: f64) -> [f64; 4] {
    let (move_min_x, move_max_x, move_min_y, move_max_y) = grip.edges();
    let [mut min_x, mut min_y, mut max_x, mut max_y] = bounds;
    if move_min_x {
        min_x += dx;
    }
    if move_max_x {
        max_x += dx;
    }
    if move_min_y {
        min_y += dy;
    }
    if move_max_y {
        max_y += dy;
    }
    if grip != RegionGrip::Body {
        if max_x - min_x < REGION_MIN_EXTENT_WORLD {
            if move_min_x {
                min_x = max_x - REGION_MIN_EXTENT_WORLD;
            } else {
                max_x = min_x + REGION_MIN_EXTENT_WORLD;
            }
        }
        if max_y - min_y < REGION_MIN_EXTENT_WORLD {
            if move_min_y {
                min_y = max_y - REGION_MIN_EXTENT_WORLD;
            } else {
                max_y = min_y + REGION_MIN_EXTENT_WORLD;
            }
        }
    }
    [min_x, min_y, max_x, max_y]
}

/// 🧲️ Quantizes one world scalar to `step`; a non-positive or absent step leaves it alone. The one
/// snap definition the area brush, the region move and the region resize all share.
pub fn snap_region_scalar(value: f64, step: Option<f64>) -> f64 {
    match step {
        Some(step) if step.is_finite() && step > 1e-9 && value.is_finite() => (value / step).round() * step,
        _ => value,
    }
}
// #endregion 🔖️TargetRegions
