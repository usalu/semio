//! 🧬️ Transparent BmpMutation aggregate.
use crate::artifacts::bmp::schema::diff::BmpDiff;
use crate::artifacts::bmp::BmpSnapshot;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::bmp::schema::operations::{apply_bmp_mutation, inverse_bmp_mutation};

//#region Owners
pub use super::change_header_fields::ChangeHeaderFieldsMutation;
pub use super::insert_palette_entry::InsertPaletteEntryMutation;
pub use super::remove_palette_entry::RemovePaletteEntryMutation;
pub use super::replace_palette_entry::ReplacePaletteEntryMutation;
pub use super::replace_pixel_data::ReplacePixelDataMutation;
//#endregion Owners

//#region Aggregate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = BmpSnapshot, diff = BmpDiff, schema = "s.stdio.bmp")]
pub enum BmpMutation {
    ChangeHeaderFields(ChangeHeaderFieldsMutation),
    InsertPaletteEntry(InsertPaletteEntryMutation),
    RemovePaletteEntry(RemovePaletteEntryMutation),
    ReplacePaletteEntry(ReplacePaletteEntryMutation),
    ReplacePixelData(ReplacePixelDataMutation),
}

//#endregion Aggregate

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<BmpMutation> {
    vec![
        crate::artifacts::bmp::schema::mutations::change_header_fields::test_case(),
        crate::artifacts::bmp::schema::mutations::insert_palette_entry::test_case(),
        crate::artifacts::bmp::schema::mutations::remove_palette_entry::test_case(),
        crate::artifacts::bmp::schema::mutations::replace_palette_entry::test_case(),
        crate::artifacts::bmp::schema::mutations::replace_pixel_data::test_case(),
    ]
}
