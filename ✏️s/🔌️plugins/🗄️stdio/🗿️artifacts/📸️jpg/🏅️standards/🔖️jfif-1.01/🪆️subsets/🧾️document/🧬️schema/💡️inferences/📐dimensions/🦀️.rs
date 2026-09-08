//! 📐 `dimensions` — one named inference: the JPEG raster's canonical geometry, a pure O(1) read
//! of already-decoded header fields — nothing here is per-entity/incremental, so this holds only
//! the value type + its pure `compute` fn (no `InferredField`).

use crate::JpgSnapshot;

//#region 🔖️Dimensions
/// 📐️ JPEG canonical raster geometry. `bit_depth` reads the SOF (T.81 §B.2.2) `precision` field
/// when a real frame has been decoded (`8` for every baseline/JFIF file this codec supports, per
/// `⚙️engine::decode_jpg`'s documented scope), falling back to the canonical `8` a freshly
/// hand-authored (`SetPixels`-only, no `frame` yet) snapshot always decodes to. `has_alpha` is
/// always `false` — JPEG (T.81) has no alpha channel, this is not a heuristic.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub has_alpha: bool,
    pub pixel_count: u64,
}

/// 📐️ Computes [`JpgDimensions`] from a snapshot's canonical/SOF fields — pure, total, O(1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_jpg_dimensions(snapshot: &JpgSnapshot) -> JpgDimensions {
    let bit_depth = snapshot.frame.as_ref().map_or(8, |frame| frame.precision as u32);
    JpgDimensions { width: snapshot.width, height: snapshot.height, bit_depth, has_alpha: false, pixel_count: snapshot.width as u64 * snapshot.height as u64 }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
