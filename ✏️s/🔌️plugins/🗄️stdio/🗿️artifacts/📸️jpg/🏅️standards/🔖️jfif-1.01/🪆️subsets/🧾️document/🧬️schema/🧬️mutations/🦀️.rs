//! 🧬️ Transparent JpgMutation aggregate.
use crate::schema::diff::JpgDiff;
use crate::JpgSnapshot;

pub use crate::schema::operations::apply_jpg_mutation;

//#region Owners
pub use super::change_jfif_header::ChangeJfifHeaderMutation;
pub use super::insert_other_segment::InsertOtherSegmentMutation;
pub use super::remove_other_segment::RemoveOtherSegmentMutation;
pub use super::replace_pixels::ReplacePixelsMutation;
pub use super::replace_image::ReplaceImage;
//#endregion Owners

//#region Aggregate

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = JpgSnapshot, diff = JpgDiff, schema = "s.stdio.jpg")]
pub enum JpgMutation {
    ChangeJfifHeader(ChangeJfifHeaderMutation),
    InsertOtherSegment(InsertOtherSegmentMutation),
    RemoveOtherSegment(RemoveOtherSegmentMutation),
    ReplacePixels(ReplacePixelsMutation),
    ReplaceImage(ReplaceImage),
}

//#endregion Aggregate

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
