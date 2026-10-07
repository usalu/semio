//! 🧬️ Transparent TiffMutation aggregate.
use crate::schema::diff::TiffDiff;
use crate::TiffSnapshot;

pub use crate::schema::operations::apply_tiff_mutation;

//#region Owners
pub use super::insert_ifd::InsertIfdMutation;
pub use super::remove_ifd::RemoveIfdMutation;
pub use super::remove_tag::RemoveTagMutation;
pub use super::replace_tag::ReplaceTagMutation;
pub use super::paint_region::PaintRegionMutation;
//#endregion Owners

//#region Aggregate
use super::set_snapshot::SetSnapshot;
use super::patch_snapshot::PatchSnapshot;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = TiffSnapshot, diff = TiffDiff, schema = "s.stdio.tiff")]
pub enum TiffMutation {
    SetSnapshot(SetSnapshot),
    PatchSnapshot(PatchSnapshot),
    InsertIfd(InsertIfdMutation),
    RemoveIfd(RemoveIfdMutation),
    ReplaceTag(ReplaceTagMutation),
    RemoveTag(RemoveTagMutation),
    PaintRegion(PaintRegionMutation),
}

//#endregion Aggregate

