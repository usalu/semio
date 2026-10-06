//! 📝️ Framing and direct codec registry for JpgMutation.
use crate::schema::mutations::JpgMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&JpgMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<JpgMutation, semio_framework_diagnostic::TextError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::patch_snapshot::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::set_snapshot::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::change_jfif_header::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_quant_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::remove_quant_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_huffman_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::remove_huffman_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::change_restart_interval::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::insert_other_segment::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::remove_other_segment::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_pixels::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::schema::mutations::change_re_encode_quality::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for JpgMutation {
    fn print_op(&self) -> String {
        REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every aggregate variant has a direct text owner")
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        let entry = REGISTRY.iter().find(|entry| entry.opcode == opcode).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown mutation opcode {opcode}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        (entry.parse)(line)
    }
}
//#endregion Framing

#[path = "📊️replace-quant/🦀️.rs"]
pub mod replace_quant;

#[path = "🪪️change-jfif/🦀️.rs"]
pub mod change_jfif;

#[path = "🧹️remove-quant/🦀️.rs"]
pub mod remove_quant;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🌳️replace-huffman/🦀️.rs"]
pub mod replace_huffman;

#[path = "🗑️remove-other/🦀️.rs"]
pub mod remove_other;

#[path = "📥️insert-other/🦀️.rs"]
pub mod insert_other;

#[path = "🎚️change-re/🦀️.rs"]
pub mod change_re;

#[path = "🔲️replace-pixels/🦀️.rs"]
pub mod replace_pixels;

#[path = "🔁️change-restart/🦀️.rs"]
pub mod change_restart;

#[path = "🪓️remove-huffman/🦀️.rs"]
pub mod remove_huffman;

#[allow(unused_imports)]
mod mutations_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::remove_huffman_table::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🪓️remove-huffman/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed remove-huffman-table payload")
}
}
pub use mutations_codec::*;


#[allow(unused_imports)]
mod mutations_wire_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::change_restart_interval::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🔁️change-restart/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed change-restart-interval payload")
}
}
pub use mutations_wire_codec::*;


#[allow(unused_imports)]
mod mutations_wire2_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_pixels::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🔲️replace-pixels/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed replace-pixels payload")
}
}
pub use mutations_wire2_codec::*;


#[allow(unused_imports)]
mod mutations_wire3_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::change_re_encode_quality::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🎚️change-re/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed change-re-encode-quality payload")
}
}
pub use mutations_wire3_codec::*;


#[allow(unused_imports)]
mod mutations_wire4_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::insert_other_segment::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/📥️insert-other/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed insert-other-segment payload")
}
}
pub use mutations_wire4_codec::*;


#[allow(unused_imports)]
mod mutations_wire5_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::remove_other_segment::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🗑️remove-other/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed remove-other-segment payload")
}
}
pub use mutations_wire5_codec::*;


#[allow(unused_imports)]
mod mutations_wire6_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_huffman_table::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🌳️replace-huffman/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed replace-huffman-table payload")
}
}
pub use mutations_wire6_codec::*;


#[allow(unused_imports)]
mod mutations_wire7_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::remove_quant_table::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🧹️remove-quant/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed remove-quant-table payload")
}
}
pub use mutations_wire7_codec::*;


#[allow(unused_imports)]
mod mutations_wire8_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::change_jfif_header::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🪪️change-jfif/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed change-jfif-header payload")
}
}
pub use mutations_wire8_codec::*;


#[allow(unused_imports)]
mod mutations_wire9_codec {
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_quant_table::*;
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/📊️replace-quant/🎯️direct/🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed replace-quant-table payload")
}
}
pub use mutations_wire9_codec::*;
