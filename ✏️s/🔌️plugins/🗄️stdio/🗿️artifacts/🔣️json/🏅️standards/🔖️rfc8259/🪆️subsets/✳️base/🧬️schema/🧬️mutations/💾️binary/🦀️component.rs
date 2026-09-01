//! 💾️ Generic framing and descriptor roster for the transparent JsonMutation.
use crate::artifacts::json::schema::mutations::JsonMutation;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
pub const BINARY_TAGS: &[(&str, u32)] = &[("set-member", 1), ("remove-member", 2), ("insert-array-element", 3), ("remove-array-element", 4), ("set-scalar", 5)];
impl protocol::OpBinary for JsonMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self))).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|cause| protocol::ProtocolError::Malformed { what: "json mutation", offset: 0, detail: cause.to_string() })?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|cause| protocol::ProtocolError::Malformed { what: "json mutation", offset: 0, detail: cause.to_string() })
    }
}
