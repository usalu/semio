//! ⚖️ EnergyModel artifact — the binary operation surface (`spr`) and its laws. Tags are the
//! declared tags in `📡️.protocol.semio`; `dsl::variants_binary` pairs each tag with the aggregate's
//! derived record body and rejects noncanonical frames.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::EnergyModelMutation;
use protocol::OpBinary;

/// 📦️ Encodes an `EnergyModelMutation` to its binary state-patch form.
pub fn encode_op(operation: &EnergyModelMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `EnergyModelMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<EnergyModelMutation, protocol::ProtocolError> {
    EnergyModelMutation::decode_op(bytes)
}

mod native_codec {
use crate::EnergyModelMutation;

dsl::original_pack_value_operation_receiving!(EnergyModelMutation);

impl protocol::OpBinary for EnergyModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = protocol::os_pack::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })
    }
}
}
