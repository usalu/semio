//! ⚖️ Remodeling artifact — the binary operation surface (`spr`) and its laws.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `RemodelingMutation` to its binary command form.
pub fn encode_op(operation: &RemodelingMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RemodelingMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RemodelingMutation, protocol::ProtocolError> {
    RemodelingMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
#[cfg(test)] use super::*;
use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;

impl protocol::OpBinary for RemodelingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = protocol::os_pack::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })
    }
}
}
