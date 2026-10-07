//! 🧬️ Transparent JpgMutation aggregate.
use crate::schema::diff::JpgDiff;
use crate::JpgSnapshot;

pub use crate::schema::operations::apply_jpg_mutation;

//#region Owners
pub use super::change_jfif_header::ChangeJfifHeaderMutation;
pub use super::change_restart_interval::ChangeRestartIntervalMutation;
pub use super::insert_other_segment::InsertOtherSegmentMutation;
pub use super::remove_huffman_table::RemoveHuffmanTableMutation;
pub use super::remove_other_segment::RemoveOtherSegmentMutation;
pub use super::remove_quant_table::RemoveQuantTableMutation;
pub use super::replace_huffman_table::ReplaceHuffmanTableMutation;
pub use super::replace_pixels::ReplacePixelsMutation;
pub use super::replace_quant_table::ReplaceQuantTableMutation;
//#endregion Owners

//#region Aggregate
use super::set_snapshot::SetSnapshot;
use super::patch_snapshot::PatchSnapshot;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = JpgSnapshot, diff = JpgDiff, schema = "s.stdio.jpg")]
pub enum JpgMutation {
    SetSnapshot(SetSnapshot),
    PatchSnapshot(PatchSnapshot),
    ChangeJfifHeader(ChangeJfifHeaderMutation),
    ReplaceQuantTable(ReplaceQuantTableMutation),
    RemoveQuantTable(RemoveQuantTableMutation),
    ReplaceHuffmanTable(ReplaceHuffmanTableMutation),
    RemoveHuffmanTable(RemoveHuffmanTableMutation),
    ChangeRestartInterval(ChangeRestartIntervalMutation),
    InsertOtherSegment(InsertOtherSegmentMutation),
    RemoveOtherSegment(RemoveOtherSegmentMutation),
    ReplacePixels(ReplacePixelsMutation),
}

//#endregion Aggregate

