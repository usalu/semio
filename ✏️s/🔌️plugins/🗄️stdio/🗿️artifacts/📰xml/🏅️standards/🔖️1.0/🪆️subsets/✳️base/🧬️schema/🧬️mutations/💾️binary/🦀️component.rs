//! 💾️ Generic framing and descriptor roster for the transparent XmlMutation.
use crate::artifacts::xml::schema::mutations::XmlMutation;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
pub const BINARY_TAGS: &[(&str, u32)] = &[("set-declaration", 1), ("set-doctype", 2), ("insert-element", 3), ("remove-element", 4), ("set-attribute", 5), ("set-text", 6)];
impl protocol::OpBinary for XmlMutation { fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(pack::to_json_string(self).into_bytes()) } fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { let text = std::str::from_utf8(bytes).map_err(|cause| protocol::ProtocolError::Malformed { what: "xml mutation", offset: 0, detail: cause.to_string() })?; pack::from_json_str(text).map_err(|cause| protocol::ProtocolError::Malformed { what: "xml mutation", offset: 0, detail: cause.to_string() }) } }
