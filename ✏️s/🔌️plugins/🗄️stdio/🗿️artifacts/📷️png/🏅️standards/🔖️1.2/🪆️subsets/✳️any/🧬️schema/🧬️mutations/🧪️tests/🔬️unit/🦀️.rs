use super::*;
use crate::schema::operations::png_revision;
use crate::schema::snapshot::{PngImage, PngNativePaint, PngRegion};

/// ⚖️ `png_mutation_inverse_sum_law`: every leaf sums its inverse diffs to the negative forward diff on a 2x2 RGBA8 image whose second row and column are the cut regions.
#[semio_framework_async_macros::async_test]
async fn png_mutation_inverse_sum_law_holds_for_every_leaf() {
    let base = PngSnapshot { image: PngImage { width: 2, height: 2, samples: (0..16).map(|word| word * 10).collect(), ..PngImage::default() }, ..PngSnapshot::default() };
    let revision = png_revision(&base);
    for mutation in [
        PngMutation::ReplaceImage(ReplaceImage { image: PngImage { samples: vec![1, 2, 3, 4], ..PngImage::default() } }),
        PngMutation::ChangeGamma(ChangeGammaMutation { revision: revision.clone(), gama: Some(45_455) }),
        PngMutation::PatchPixels(PatchPixelsMutation { revision: revision.clone(), x: 1, y: 0, width: 1, height: 2, red: 9, green: 8, blue: 7, alpha: 200 }),
        PngMutation::PaintNativeSamples(PaintNativeSamplesMutation { revision, region: PngRegion { x: 0, y: 1, width: 2, height: 1 }, paint: PngNativePaint::rgba(1, 2, 3, 4) }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
