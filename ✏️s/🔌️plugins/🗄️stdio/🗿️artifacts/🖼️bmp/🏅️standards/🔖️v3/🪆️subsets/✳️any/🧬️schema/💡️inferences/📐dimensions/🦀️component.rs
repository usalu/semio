//! 📐 `dimensions` — one named inference: the BMP raster's BITMAPINFOHEADER geometry, a pure
//! O(1) read of already-decoded header fields — nothing here is per-entity/incremental, so this
//! holds only the value type + its pure `compute` fn (no `InferredField`).

use crate::artifacts::bmp::BmpSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Dimensions
/// 📐️ BMP BITMAPINFOHEADER-derived raster geometry. `has_alpha` is a documented heuristic, not
/// exact: `BmpSnapshot` retains `bits_per_pixel` but not the BI_BITFIELDS alpha mask
/// `⚙️engine::decode_bmp` reads transiently (§ decode: `masks[3] != 0`) — `32`bpp is the closest
/// honest proxy this snapshot's own persisted fields support (this codec's own `BI_RGB` default
/// for 32bpp carries no alpha, `⚙️engine`'s own doc comment; a real `BI_BITFIELDS` alpha mask would
/// flip this true, but that bit isn't retained on the snapshot to check).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    BmpDimensions { width: snapshot.width, height: snapshot.height, bit_depth: snapshot.bits_per_pixel, has_alpha: snapshot.bits_per_pixel == 32, pixel_count: snapshot.width as u64 * snapshot.height as u64 }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn derives_from_header_fields() {
        let snapshot = BmpSnapshot { width: 5, height: 2, bits_per_pixel: 24, ..BmpSnapshot::default() };
        assert_eq!(compute_bmp_dimensions(&snapshot), BmpDimensions { width: 5, height: 2, bit_depth: 24, has_alpha: false, pixel_count: 10 });
    }

    #[semio_framework_async_macros::async_test]
    async fn thirty_two_bpp_is_treated_as_alpha_capable() {
        let snapshot = BmpSnapshot { width: 1, height: 1, bits_per_pixel: 32, ..BmpSnapshot::default() };
        assert!(compute_bmp_dimensions(&snapshot).has_alpha);
    }
}
//#endregion 🧪️Tests
