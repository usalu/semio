use super::*;
use crate::schema::snapshot::{TiffFieldType, TiffIfd, TiffTag, TiffValues, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot_with_tags(tags: Vec<TiffTag>) -> TiffSnapshot {
    TiffSnapshot { ifds: vec![TiffIfd { pixels: Vec::new(), entries: tags }], ..TiffSnapshot::default() }
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
