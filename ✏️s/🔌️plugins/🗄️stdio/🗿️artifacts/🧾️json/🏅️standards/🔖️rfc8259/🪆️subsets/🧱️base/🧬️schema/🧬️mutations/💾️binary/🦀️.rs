//! 💾️ Generic framing and descriptor roster for the transparent JsonMutation.
use crate::schema::mutations::JsonMutation;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub const BINARY_TAGS: &[(&str, u32)] = &[("set-member", 1), ("remove-member", 2), ("insert-array-element", 3), ("remove-array-element", 4), ("set-scalar", 5)];
//#region 🏷️WireTags
/// 🏷️ `JsonMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
//#endregion 🏷️WireTags

impl protocol::OpBinary for JsonMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}
