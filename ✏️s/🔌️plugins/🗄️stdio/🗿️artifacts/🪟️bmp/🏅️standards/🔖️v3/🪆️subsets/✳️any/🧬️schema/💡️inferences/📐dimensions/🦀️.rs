//! 📐 `dimensions` — one named inference: the BMP raster's BITMAPINFOHEADER geometry, a pure
//! O(1) read of already-decoded header fields — nothing here is per-entity/incremental, so this
//! holds only the value type + its pure `compute` fn (no `InferredField`).

use crate::BmpSnapshot;

//#region 🔖️Dimensions
/// 📐️ Exact raster geometry and alpha lane presence from the owned native image.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BmpDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u16,
    pub has_alpha: bool,
    pub pixel_count: u64,
}

/// 📐️ Computes [`BmpDimensions`] from a snapshot's header fields — pure, total, O(1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_bmp_dimensions(snapshot: &BmpSnapshot) -> BmpDimensions {
    let image = &snapshot.image;
    BmpDimensions { width: image.width, height: image.height, bit_depth: image.profile.bits_per_pixel(), has_alpha: image.masks[3] != 0, pixel_count: u64::from(image.width) * u64::from(image.height) }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
