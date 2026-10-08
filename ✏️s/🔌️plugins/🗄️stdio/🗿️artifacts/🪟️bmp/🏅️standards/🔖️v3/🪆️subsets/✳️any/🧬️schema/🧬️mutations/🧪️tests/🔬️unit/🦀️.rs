use super::*;
use crate::schema::operations::bmp_revision;
use crate::schema::snapshot::{BmpImage, BmpNativeSample, BmpPixels, BmpRegion};
use protocol::MutationKind;

/// ⚖️ `bmp_mutation_inverse_sum_law`: every leaf sums its inverse diffs to the negative forward diff on a 2x2 direct-colour image whose second column is the painted region.
#[semio_framework_async_macros::async_test]
async fn bmp_mutation_inverse_sum_law_holds_for_every_leaf() {
    let pixel = |shade: u32| BmpNativeSample { red: shade, green: shade, blue: shade, alpha: 255, reserved: 0 };
    let base = BmpSnapshot { image: BmpImage { width: 2, height: 2, pixels: BmpPixels::Direct { samples: vec![pixel(1), pixel(2), pixel(3), pixel(4)] }, ..BmpImage::default() }, ..BmpSnapshot::default() };
    let revision = bmp_revision(&base);
    for mutation in [
        BmpMutation::ReplaceImage(ReplaceImage { image: BmpImage::default() }),
        BmpMutation::PaintDirectRegion(PaintDirectRegion { revision, x: 1, y: 0, width: 1, height: 2, red: 9, green: 8, blue: 7, alpha: 255 }),
        BmpMutation::ReplaceSamples(ReplaceSamples { region: BmpRegion { x: 0, y: 1, width: 2, height: 1 }, indices: Vec::new(), samples: vec![pixel(7), pixel(8)] }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

/// 🔎 A paint's inverse is one `replace-samples` carrying exactly the base samples of the painted region, and its diff holds only that rectangle.
#[test]
fn bmp_paint_is_a_sparse_region_row_with_a_region_inverse() {
    let pixel = |shade: u32| BmpNativeSample { red: shade, green: shade, blue: shade, alpha: 255, reserved: 0 };
    let base = BmpSnapshot { image: BmpImage { width: 2, height: 2, pixels: BmpPixels::Direct { samples: vec![pixel(1), pixel(2), pixel(3), pixel(4)] }, ..BmpImage::default() }, ..BmpSnapshot::default() };
    let paint = PaintDirectRegion { revision: bmp_revision(&base), x: 1, y: 0, width: 1, height: 2, red: 9, green: 8, blue: 7, alpha: 255 };
    let outcome = paint.diff(&base);
    let diff = outcome.diff();
    assert!(diff.image.is_none());
    assert_eq!(diff.rects.len(), 1);
    assert_eq!(diff.rects[0].region, BmpRegion { x: 1, y: 0, width: 1, height: 2 });
    assert_eq!(paint.inverse(&base).unwrap(), vec![BmpMutation::ReplaceSamples(ReplaceSamples { region: BmpRegion { x: 1, y: 0, width: 1, height: 2 }, indices: Vec::new(), samples: vec![pixel(2), pixel(4)] })]);
}
