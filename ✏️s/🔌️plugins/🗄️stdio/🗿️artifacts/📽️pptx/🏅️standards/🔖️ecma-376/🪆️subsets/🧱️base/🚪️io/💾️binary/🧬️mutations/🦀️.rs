//! pptx rep for stdio.pptx 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::*;
use crate::schema::diff::PptxDiff;
use crate::schema::snapshot::{PptxParagraph, PptxShape, PptxSlide, PptxTransform};
use crate::PptxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use xml_address::{PptxShapeAddress, PptxSlideAddress, PptxXmlAddress, PptxXmlVacancyAddress};

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F1's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `PptxMutation` variant ordinal, in the same 0-8 order `print_pptx_mutation`'s own keyword
/// match uses.
impl OpBinary for PptxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ `PptxMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
//#endregion 🏷️WireTags
