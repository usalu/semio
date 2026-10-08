//! 📝️ Framing and direct codec registry for TiffMutation.
use crate::schema::mutations::TiffMutation;

//#region Registry
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&TiffMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<TiffMutation, semio_framework_diagnostic::TextError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::standards::v6_0::subsets::document::io::text::mutations::insert_ifd::CODEC,
    crate::standards::v6_0::subsets::document::io::text::mutations::remove_ifd::CODEC,
    crate::standards::v6_0::subsets::document::io::text::mutations::replace_tag::CODEC,
    crate::standards::v6_0::subsets::document::io::text::mutations::remove_tag::CODEC,
    crate::standards::v6_0::subsets::document::io::text::mutations::paint_region::CODEC,
    crate::standards::v6_0::subsets::document::io::text::mutations::replace_samples::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpText for TiffMutation {
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


#[path = "🗑️remove-tag/🦀️.rs"]
pub mod remove_tag;

#[path = "📥️insert-ifd/🦀️.rs"]
pub mod insert_ifd;

#[path = "🏷️replace-tag/🦀️.rs"]
pub mod replace_tag;

#[path = "📤️remove-ifd/🦀️.rs"]
pub mod remove_ifd;

#[path = "🎨️paint-region/🦀️.rs"]
pub mod paint_region;

#[path = "🧮️replace-samples/🦀️.rs"]
pub mod replace_samples;

#[allow(unused_imports)]
mod mutations_codec {
use crate::standards::v6_0::subsets::document::schema::mutations::remove_ifd::*;
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn remove_ifd_test_case() -> TiffMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/📤️remove-ifd/🎯️direct-behavior/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed remove-ifd payload")
}
}
pub use mutations_codec::*;


#[allow(unused_imports)]
mod mutations_wire_codec {
use crate::standards::v6_0::subsets::document::schema::mutations::replace_tag::*;
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn replace_tag_test_case() -> TiffMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🏷️replace-tag/🎯️direct-behavior/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed replace-tag payload")
}
}
pub use mutations_wire_codec::*;


#[allow(unused_imports)]
mod mutations_wire2_codec {
use crate::standards::v6_0::subsets::document::schema::mutations::insert_ifd::*;
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn insert_ifd_test_case() -> TiffMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/📥️insert-ifd/🎯️direct-behavior/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed insert-ifd payload")
}
}
pub use mutations_wire2_codec::*;


#[allow(unused_imports)]
mod mutations_wire3_codec {
use crate::standards::v6_0::subsets::document::schema::mutations::remove_tag::*;
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

#[cfg(test)]
pub(crate) fn remove_tag_test_case() -> TiffMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🗑️remove-tag/🎯️direct-behavior/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed remove-tag payload")
}
}
pub use mutations_wire3_codec::*;


#[cfg(test)]
pub(crate) fn replace_samples_test_case() -> TiffMutation {
    semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🧮️replace-samples/🎯️direct-behavior/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed replace-samples payload")
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<TiffMutation> {
    vec![
        insert_ifd_test_case(),
        remove_ifd_test_case(),
        replace_tag_test_case(),
        remove_tag_test_case(),
        crate::schema::mutations::paint_region::test_case(),
        replace_samples_test_case(),
    ]
}
