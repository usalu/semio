use super::*;
use crate::schema::operations::bmp_revision;
use crate::schema::snapshot::{BmpImage, BmpNativeSample, BmpPixels};

/// ⚖️ `bmp_mutation_inverse_sum_law`: every leaf sums its inverse diffs to the negative forward diff on a 2x2 direct-colour image whose second column is the painted region.
#[semio_framework_async_macros::async_test]
async fn bmp_mutation_inverse_sum_law_holds_for_every_leaf() {
    let pixel = |shade: u32| BmpNativeSample { red: shade, green: shade, blue: shade, alpha: 255, reserved: 0 };
    let base = BmpSnapshot { image: BmpImage { width: 2, height: 2, pixels: BmpPixels::Direct { samples: vec![pixel(1), pixel(2), pixel(3), pixel(4)] }, ..BmpImage::default() }, ..BmpSnapshot::default() };
    let revision = bmp_revision(&base);
    for mutation in [
        BmpMutation::ReplaceImage(ReplaceImage { image: BmpImage::default() }),
        BmpMutation::PaintDirectRegion(PaintDirectRegion { revision, x: 1, y: 0, width: 1, height: 2, red: 9, green: 8, blue: 7, alpha: 255 }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
