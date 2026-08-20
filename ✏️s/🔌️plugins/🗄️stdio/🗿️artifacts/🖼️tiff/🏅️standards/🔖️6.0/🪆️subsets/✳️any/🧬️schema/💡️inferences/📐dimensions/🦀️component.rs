//! 📐 `dimensions` — one named inference: the TIFF raster's baseline-tag-derived geometry, a
//! pure O(1) read of already-decoded IFD 0 tags — nothing here is per-entity/incremental, so this
//! holds only the value type + its pure `compute` fn (no `InferredField`).

use crate::artifacts::tiff::schema::snapshot::{TAG_BITS_PER_SAMPLE, TAG_SAMPLES_PER_PIXEL};
use crate::artifacts::tiff::TiffSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Dimensions
/// 📐️ TIFF baseline-tag-derived raster geometry (TIFF6 §8/§19). `bit_depth` reads
/// `BitsPerSample`(258)'s first value (TIFF6 §19's own precedent for "the" bit depth of a
/// possibly-multi-sample image), defaulting to `1` — TIFF6 §8's own documented default for an
/// absent `BitsPerSample` tag. `has_alpha` is a documented heuristic, not exact: this snapshot
/// retains `SamplesPerPixel`(277) but not `ExtraSamples`(338) (never decoded by this codec, see
/// `⚙️engine`), so `samplesPerPixel > 3` (more channels than plain RGB) is the closest honest
/// proxy available.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub has_alpha: bool,
    pub pixel_count: u64,
}

/// 📐️ Computes [`TiffDimensions`] from a snapshot's IFD 0 tags — pure, total, O(1).
pub async fn compute_tiff_dimensions(snapshot: &TiffSnapshot) -> TiffDimensions {
    let width = snapshot.width().await.unwrap_or(0);
    let height = snapshot.height().await.unwrap_or(0);
    let bit_depth = snapshot.tag(TAG_BITS_PER_SAMPLE).await.and_then(|tag| tag.values.first_u32()).unwrap_or(1);
    let samples_per_pixel = snapshot.tag(TAG_SAMPLES_PER_PIXEL).await.and_then(|tag| tag.values.first_u32());
    let has_alpha = samples_per_pixel.map(|samples| samples > 3).unwrap_or(false);
    TiffDimensions { width, height, bit_depth, has_alpha, pixel_count: width as u64 * height as u64 }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::tiff::schema::snapshot::{TiffFieldType, TiffIfd, TiffTag, TiffValues, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH};

    async fn snapshot_with_tags(tags: Vec<TiffTag>) -> TiffSnapshot {
        TiffSnapshot { ifds: vec![TiffIfd { entries: tags }], ..TiffSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn derives_from_baseline_tags() {
        let snapshot = snapshot_with_tags(vec![
            TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) },
            TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![3]) },
            TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8, 8, 8]) },
            TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) },
        ]);
        let dims = compute_tiff_dimensions(&snapshot);
        assert_eq!(dims, TiffDimensions { width: 4, height: 3, bit_depth: 8, has_alpha: false, pixel_count: 12 });
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_bits_per_sample_falls_back_to_one() {
        assert_eq!(compute_tiff_dimensions(&TiffSnapshot::default()).bit_depth, 1);
    }
}
//#endregion 🧪️Tests
