//! 📐 `dimensions` — one named inference: the PNG raster's header geometry (IHDR §11.2.2), a
//! pure O(1) read of already-decoded header fields — nothing here is per-entity/incremental, so
//! this holds only the value type + its pure `compute` fn (no `InferredField`).

use crate::artifacts::png::schema::snapshot::PngColorType;
use crate::artifacts::png::PngSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Dimensions
/// 📐️ PNG IHDR-derived raster geometry. `has_alpha` is exact (PNG's `colorType` is an explicit
/// enum, §11.2.2) — unlike jpg/bmp/tiff this needs no heuristic.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub has_alpha: bool,
    pub pixel_count: u64,
}

/// 📐️ Computes [`PngDimensions`] from a snapshot's IHDR fields — pure, total, O(1).
pub async fn compute_png_dimensions(snapshot: &PngSnapshot) -> PngDimensions {
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
mod tests {
    use super::*;

    #[test]
    async fn derives_from_ihdr_fields() {
        let snapshot = PngSnapshot { width: 4, height: 3, bit_depth: 8, color_type: PngColorType::Rgba, ..PngSnapshot::default() };
        assert_eq!(compute_png_dimensions(&snapshot), PngDimensions { width: 4, height: 3, bit_depth: 8, has_alpha: true, pixel_count: 12 });
    }

    #[test]
    async fn rgb_has_no_alpha() {
        let snapshot = PngSnapshot { width: 2, height: 2, bit_depth: 8, color_type: PngColorType::Rgb, ..PngSnapshot::default() };
        assert!(!compute_png_dimensions(&snapshot).has_alpha);
    }
}
//#endregion 🧪️Tests
