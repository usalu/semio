//! 📐 `dimensions` — one named inference: the TIFF raster's baseline-tag-derived geometry, a
//! pure O(1) read of already-decoded IFD 0 tags — nothing here is per-entity/incremental, so this
//! holds only the value type + its pure `compute` fn (no `InferredField`).

use crate::schema::snapshot::{TAG_BITS_PER_SAMPLE, TAG_SAMPLES_PER_PIXEL};
use crate::TiffSnapshot;

//#region 🔖️Dimensions
/// 📐️ TIFF baseline-tag-derived raster geometry (TIFF6 §8/§19). `bit_depth` reads
/// `BitsPerSample`(258)'s first value (TIFF6 §19's own precedent for "the" bit depth of a
/// possibly-multi-sample image), defaulting to `1` — TIFF6 §8's own documented default for an
/// absent `BitsPerSample` tag. `has_alpha` is a documented heuristic, not exact: this snapshot
/// retains `SamplesPerPixel`(277) but not `ExtraSamples`(338) (never decoded by this codec, see
/// `⚙️engine`), so `samplesPerPixel > 3` (more channels than plain RGB) is the closest honest
/// proxy available.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub has_alpha: bool,
    pub pixel_count: u64,
}

/// 📐️ Computes [`TiffDimensions`] from a snapshot's IFD 0 tags — pure, total, O(1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_tiff_dimensions(snapshot: &TiffSnapshot) -> TiffDimensions {
    let width = snapshot.width().unwrap_or(0);
    let height = snapshot.height().unwrap_or(0);
    let bit_depth = snapshot.tag(TAG_BITS_PER_SAMPLE).and_then(|tag| tag.values.first_u32()).unwrap_or(1);
    let samples_per_pixel = snapshot.tag(TAG_SAMPLES_PER_PIXEL).and_then(|tag| tag.values.first_u32());
    let has_alpha = samples_per_pixel.is_some_and(|samples| samples > 3);
    TiffDimensions { width, height, bit_depth, has_alpha, pixel_count: width as u64 * height as u64 }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
