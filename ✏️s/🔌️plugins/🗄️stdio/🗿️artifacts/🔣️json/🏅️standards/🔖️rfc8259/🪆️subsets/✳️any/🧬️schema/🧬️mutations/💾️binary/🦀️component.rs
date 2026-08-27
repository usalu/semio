//! 💾️ Generic framing and descriptor roster for the transparent JsonMutation.
use crate::artifacts::json::schema::mutations::JsonMutation;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
pub const BINARY_TAGS: &[(&str, u32)] = &[("set-member", 1), ("remove-member", 2), ("insert-array-element", 3), ("remove-array-element", 4), ("set-scalar", 5)];
impl protocol::OpBinary for JsonMutation { fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { serde_json::to_vec(self).map_err(|cause| protocol::ProtocolError::Malformed { what: "json mutation", offset: 0, detail: cause.to_string() }) } fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { serde_json::from_slice(bytes).map_err(|cause| protocol::ProtocolError::Malformed { what: "json mutation", offset: 0, detail: cause.to_string() }) } }
