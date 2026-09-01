//! 🧬️ Transparent PngMutation aggregate.
use crate::artifacts::png::schema::diff::PngDiff;
use crate::artifacts::png::PngSnapshot;

pub use crate::artifacts::png::schema::operations::{apply_png_mutation, inverse_png_mutation};

//#region Owners
pub use super::change_background::ChangeBackgroundMutation;
pub use super::change_chromaticities::ChangeChromaticitiesMutation;
pub use super::change_gamma::ChangeGammaMutation;
pub use super::change_header::ChangeHeaderMutation;
pub use super::change_physical_dims::ChangePhysicalDimsMutation;
pub use super::change_srgb_intent::ChangeSrgbIntentMutation;
pub use super::change_timestamp::ChangeTimestampMutation;
pub use super::change_transparency::ChangeTransparencyMutation;
pub use super::insert_text_chunk::InsertTextChunkMutation;
pub use super::insert_unknown_chunk::InsertUnknownChunkMutation;
pub use super::remove_text_chunk::RemoveTextChunkMutation;
pub use super::remove_unknown_chunk::RemoveUnknownChunkMutation;
pub use super::replace_palette::ReplacePaletteMutation;
pub use super::replace_pixels::ReplacePixelsMutation;
pub use super::replace_text_chunk::ReplaceTextChunkMutation;
//#endregion Owners

//#region Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = PngSnapshot, diff = PngDiff, schema = "s.stdio.png")]
pub enum PngMutation {
    ChangeHeader(ChangeHeaderMutation),
    ReplacePalette(ReplacePaletteMutation),
    ChangeTransparency(ChangeTransparencyMutation),
    ChangeGamma(ChangeGammaMutation),
    ChangeChromaticities(ChangeChromaticitiesMutation),
    ChangeSrgbIntent(ChangeSrgbIntentMutation),
    ChangePhysicalDims(ChangePhysicalDimsMutation),
    ChangeTimestamp(ChangeTimestampMutation),
    ChangeBackground(ChangeBackgroundMutation),
    InsertTextChunk(InsertTextChunkMutation),
    RemoveTextChunk(RemoveTextChunkMutation),
    ReplaceTextChunk(ReplaceTextChunkMutation),
    ReplacePixels(ReplacePixelsMutation),
    InsertUnknownChunk(InsertUnknownChunkMutation),
    RemoveUnknownChunk(RemoveUnknownChunkMutation),
}

//#endregion Aggregate

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<PngMutation> {
    vec![
        crate::artifacts::png::schema::mutations::change_header::test_case(),
        crate::artifacts::png::schema::mutations::replace_palette::test_case(),
        crate::artifacts::png::schema::mutations::change_transparency::test_case(),
        crate::artifacts::png::schema::mutations::change_gamma::test_case(),
        crate::artifacts::png::schema::mutations::change_chromaticities::test_case(),
        crate::artifacts::png::schema::mutations::change_srgb_intent::test_case(),
        crate::artifacts::png::schema::mutations::change_physical_dims::test_case(),
        crate::artifacts::png::schema::mutations::change_timestamp::test_case(),
        crate::artifacts::png::schema::mutations::change_background::test_case(),
        crate::artifacts::png::schema::mutations::insert_text_chunk::test_case(),
        crate::artifacts::png::schema::mutations::remove_text_chunk::test_case(),
        crate::artifacts::png::schema::mutations::replace_text_chunk::test_case(),
        crate::artifacts::png::schema::mutations::replace_pixels::test_case(),
        crate::artifacts::png::schema::mutations::insert_unknown_chunk::test_case(),
        crate::artifacts::png::schema::mutations::remove_unknown_chunk::test_case(),
    ]
}
