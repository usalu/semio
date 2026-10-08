use super::*;
use crate::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgImage, JpgSegment};

/// ⚖️ `jpg_mutation_inverse_sum_law`: every leaf sums its inverse diffs to the negative forward diff on an image whose other-segment list has MIDDLE entries to cut and insert at.
#[semio_framework_async_macros::async_test]
async fn jpg_mutation_inverse_sum_law_holds_for_every_leaf() {
    let segment = |marker: u8, data: Vec<u8>| JpgSegment { marker, data };
    let base = JpgSnapshot { image: JpgImage { width: 2, height: 1, pixels: vec![1, 2, 3, 4, 5, 6], other_segments: vec![segment(0xFE, vec![1]), segment(0xE1, vec![2, 2]), segment(0xE2, vec![3])], ..JpgImage::default() }, ..JpgSnapshot::default() };
    let thumbnail = JfifThumbnail { width: 1, height: 1, rgb_data: vec![7, 8, 9] };
    for mutation in [
        JpgMutation::InsertOtherSegment(InsertOtherSegmentMutation { index: 1, segment: segment(0xE3, vec![9]) }),
        JpgMutation::InsertOtherSegment(InsertOtherSegmentMutation { index: 3, segment: segment(0xE4, vec![8]) }),
        JpgMutation::RemoveOtherSegment(RemoveOtherSegmentMutation { index: 0 }),
        JpgMutation::RemoveOtherSegment(RemoveOtherSegmentMutation { index: 1 }),
        JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels: vec![9, 9, 9, 9, 9, 9] }),
        JpgMutation::ChangeJfifHeader(ChangeJfifHeaderMutation { version: (1, 2), density_units: JfifDensityUnits::PixelsPerInch, x_density: 72, y_density: 72, thumbnail: Some(thumbnail) }),
        JpgMutation::ReplaceImage(ReplaceImage { image: JpgImage { width: 1, height: 1, pixels: vec![9, 9, 9], other_segments: vec![segment(0xFE, vec![5])], ..JpgImage::default() } }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
