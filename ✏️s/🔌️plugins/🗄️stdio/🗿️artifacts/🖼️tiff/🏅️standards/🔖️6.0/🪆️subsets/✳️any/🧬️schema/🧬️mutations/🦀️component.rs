//! 🧬️ Transparent TiffMutation aggregate.
use crate::artifacts::tiff::schema::diff::TiffDiff;
use crate::artifacts::tiff::TiffSnapshot;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::tiff::schema::operations::{apply_tiff_mutation, inverse_tiff_mutation};

//#region Owners
pub use super::change_byte_order::ChangeByteOrderMutation;
pub use super::insert_ifd::InsertIfdMutation;
pub use super::remove_ifd::RemoveIfdMutation;
pub use super::remove_tag::RemoveTagMutation;
pub use super::replace_pixels::ReplacePixelsMutation;
pub use super::replace_tag::ReplaceTagMutation;
//#endregion Owners

//#region Aggregate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = TiffSnapshot, diff = TiffDiff, schema = "s.stdio.tiff")]
pub enum TiffMutation {
    ChangeByteOrder(ChangeByteOrderMutation),
    InsertIfd(InsertIfdMutation),
    RemoveIfd(RemoveIfdMutation),
    ReplaceTag(ReplaceTagMutation),
    RemoveTag(RemoveTagMutation),
    ReplacePixels(ReplacePixelsMutation),
}

//#endregion Aggregate

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<TiffMutation> {
    vec![
        crate::artifacts::tiff::schema::mutations::change_byte_order::test_case(),
        crate::artifacts::tiff::schema::mutations::insert_ifd::test_case(),
        crate::artifacts::tiff::schema::mutations::remove_ifd::test_case(),
        crate::artifacts::tiff::schema::mutations::replace_tag::test_case(),
        crate::artifacts::tiff::schema::mutations::remove_tag::test_case(),
        crate::artifacts::tiff::schema::mutations::replace_pixels::test_case(),
    ]
}
