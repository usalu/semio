//! 🧬️ Transparent JpgMutation aggregate.
use crate::artifacts::jpg::schema::diff::JpgDiff;
use crate::artifacts::jpg::JpgSnapshot;

pub use crate::artifacts::jpg::schema::operations::{apply_jpg_mutation, inverse_jpg_mutation};

//#region Owners
pub use super::change_jfif_header::ChangeJfifHeaderMutation;
pub use super::change_re_encode_quality::ChangeReEncodeQualityMutation;
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = JpgSnapshot, diff = JpgDiff, schema = "s.stdio.jpg")]
pub enum JpgMutation {
    ChangeJfifHeader(ChangeJfifHeaderMutation),
    ReplaceQuantTable(ReplaceQuantTableMutation),
    RemoveQuantTable(RemoveQuantTableMutation),
    ReplaceHuffmanTable(ReplaceHuffmanTableMutation),
    RemoveHuffmanTable(RemoveHuffmanTableMutation),
    ChangeRestartInterval(ChangeRestartIntervalMutation),
    InsertOtherSegment(InsertOtherSegmentMutation),
    RemoveOtherSegment(RemoveOtherSegmentMutation),
    ReplacePixels(ReplacePixelsMutation),
    ChangeReEncodeQuality(ChangeReEncodeQualityMutation),
}

//#endregion Aggregate

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<JpgMutation> {
    vec![
        crate::artifacts::jpg::schema::mutations::change_jfif_header::test_case(),
        crate::artifacts::jpg::schema::mutations::replace_quant_table::test_case(),
        crate::artifacts::jpg::schema::mutations::remove_quant_table::test_case(),
        crate::artifacts::jpg::schema::mutations::replace_huffman_table::test_case(),
        crate::artifacts::jpg::schema::mutations::remove_huffman_table::test_case(),
        crate::artifacts::jpg::schema::mutations::change_restart_interval::test_case(),
        crate::artifacts::jpg::schema::mutations::insert_other_segment::test_case(),
        crate::artifacts::jpg::schema::mutations::remove_other_segment::test_case(),
        crate::artifacts::jpg::schema::mutations::replace_pixels::test_case(),
        crate::artifacts::jpg::schema::mutations::change_re_encode_quality::test_case(),
    ]
}
