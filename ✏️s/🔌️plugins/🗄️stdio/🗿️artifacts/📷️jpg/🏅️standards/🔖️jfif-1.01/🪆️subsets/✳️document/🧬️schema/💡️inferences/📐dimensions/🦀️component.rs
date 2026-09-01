//! 📐 `dimensions` — one named inference: the JPEG raster's canonical geometry, a pure O(1) read
//! of already-decoded header fields — nothing here is per-entity/incremental, so this holds only
//! the value type + its pure `compute` fn (no `InferredField`).

use crate::artifacts::jpg::JpgSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Dimensions
/// 📐️ JPEG canonical raster geometry. `bit_depth` reads the SOF (T.81 §B.2.2) `precision` field
/// when a real frame has been decoded (`8` for every baseline/JFIF file this codec supports, per
/// `⚙️engine::decode_jpg`'s documented scope), falling back to the canonical `8` a freshly
/// hand-authored (`SetPixels`-only, no `frame` yet) snapshot always decodes to. `has_alpha` is
/// always `false` — JPEG (T.81) has no alpha channel, this is not a heuristic.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    let bit_depth = snapshot.frame.as_ref().map(|frame| frame.precision as u32).unwrap_or(8);
    JpgDimensions { width: snapshot.width, height: snapshot.height, bit_depth, has_alpha: false, pixel_count: snapshot.width as u64 * snapshot.height as u64 }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn falls_back_to_canonical_8bit_precision_without_a_decoded_frame() {
        let snapshot = JpgSnapshot { width: 4, height: 2, ..JpgSnapshot::default() };
        assert_eq!(compute_jpg_dimensions(&snapshot), JpgDimensions { width: 4, height: 2, bit_depth: 8, has_alpha: false, pixel_count: 8 });
    }

    #[semio_framework_async_macros::async_test]
    async fn never_reports_alpha() {
        assert!(!compute_jpg_dimensions(&JpgSnapshot::default()).has_alpha);
    }
}
//#endregion 🧪️Tests
