//! 💾️ Generic framing and descriptor roster for the transparent XmlMutation.
use crate::schema::mutations::XmlMutation;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub const BINARY_TAGS: &[(&str, u32)] = &[("set-declaration", 1), ("set-doctype", 2), ("insert-element", 3), ("remove-element", 4), ("set-attribute", 5), ("set-text", 6)];
//#region 🏷️WireTags
/// 🏷️ `XmlMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
//#endregion 🏷️WireTags

impl protocol::OpBinary for XmlMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}
