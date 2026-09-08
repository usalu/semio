//! 📐 `dimensions` — one named inference: the PNG raster's header geometry (IHDR §11.2.2), a
//! pure O(1) read of already-decoded header fields — nothing here is per-entity/incremental, so
//! this holds only the value type + its pure `compute` fn (no `InferredField`).

use crate::schema::snapshot::PngColorType;
use crate::PngSnapshot;

//#region 🔖️Dimensions
/// 📐️ PNG IHDR-derived raster geometry. `has_alpha` is exact (PNG's `colorType` is an explicit
/// enum, §11.2.2) — unlike jpg/bmp/tiff this needs no heuristic.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PngDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub has_alpha: bool,
    pub pixel_count: u64,
}

/// 📐️ Computes [`PngDimensions`] from a snapshot's IHDR fields — pure, total, O(1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_png_dimensions(snapshot: &PngSnapshot) -> PngDimensions {
    PngDimensions {
        width: snapshot.width,
        height: snapshot.height,
        bit_depth: snapshot.bit_depth,
        has_alpha: matches!(snapshot.color_type, PngColorType::GrayscaleAlpha | PngColorType::Rgba),
        pixel_count: snapshot.width as u64 * snapshot.height as u64,
    }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
