//! 🧬️ PNG mutation aggregate over precise owned image values.

use crate::schema::diff::PngDiff;
use crate::PngSnapshot;

pub use super::change_gamma::ChangeGammaMutation;
pub use super::patch_pixels::PatchPixelsMutation;
pub use super::paint_native_samples::PaintNativeSamplesMutation;
pub use super::replace_image::ReplaceImage;
pub use crate::schema::operations::apply_png_mutation;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = PngSnapshot, diff = PngDiff, schema = "s.stdio.png")]
pub enum PngMutation {
    ReplaceImage(ReplaceImage),
    ChangeGamma(ChangeGammaMutation),
    PatchPixels(PatchPixelsMutation),
    PaintNativeSamples(PaintNativeSamplesMutation),
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<PngMutation> {
    vec![
        super::change_gamma::test_case(),
        super::patch_pixels::test_case(),
        super::paint_native_samples::test_case(),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
