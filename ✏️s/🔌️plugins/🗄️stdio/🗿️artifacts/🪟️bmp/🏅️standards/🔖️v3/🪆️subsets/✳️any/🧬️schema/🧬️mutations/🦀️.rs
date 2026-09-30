//! 🧬️ Transparent BmpMutation aggregate.
use crate::schema::diff::BmpDiff;
use crate::BmpSnapshot;

pub use crate::schema::operations::{apply_bmp_mutation, decode_bmp_mutation_payload, inverse_bmp_mutation};

//#region Owners
pub use super::change_header_fields::ChangeHeaderFieldsMutation;
pub use super::insert_palette_entry::InsertPaletteEntryMutation;
pub use super::remove_palette_entry::RemovePaletteEntryMutation;
pub use super::replace_palette_entry::ReplacePaletteEntryMutation;
pub use super::replace_pixel_data::ReplacePixelDataMutation;
pub use super::set_snapshot::SetSnapshot;
//#endregion Owners

//#region Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = BmpSnapshot, diff = BmpDiff, schema = "s.stdio.bmp")]
pub enum BmpMutation {
    SetSnapshot(SetSnapshot),
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
        BmpMutation::SetSnapshot(SetSnapshot { snapshot: BmpSnapshot::default() }),
        crate::schema::mutations::change_header_fields::test_case(),
        crate::schema::mutations::insert_palette_entry::test_case(),
        crate::schema::mutations::remove_palette_entry::test_case(),
        crate::schema::mutations::replace_palette_entry::test_case(),
        crate::schema::mutations::replace_pixel_data::test_case(),
    ]
}
