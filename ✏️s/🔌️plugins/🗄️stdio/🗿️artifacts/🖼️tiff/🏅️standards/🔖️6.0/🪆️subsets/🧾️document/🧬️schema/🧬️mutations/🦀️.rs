//! 🧬️ Transparent TiffMutation aggregate.
use crate::schema::diff::TiffDiff;
use crate::TiffSnapshot;



//#region Owners
pub use super::insert_ifd::InsertIfdMutation;
pub use super::remove_ifd::RemoveIfdMutation;
pub use super::remove_tag::RemoveTagMutation;
pub use super::replace_samples::ReplaceSamplesMutation;
pub use super::replace_tag::ReplaceTagMutation;
pub use super::paint_region::PaintRegionMutation;
//#endregion Owners

//#region Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = TiffSnapshot, diff = TiffDiff, schema = "s.stdio.tiff")]
pub enum TiffMutation {
    InsertIfd(InsertIfdMutation),
    RemoveIfd(RemoveIfdMutation),
    ReplaceTag(ReplaceTagMutation),
    RemoveTag(RemoveTagMutation),
    PaintRegion(PaintRegionMutation),
    ReplaceSamples(ReplaceSamplesMutation),
}

//#endregion Aggregate

