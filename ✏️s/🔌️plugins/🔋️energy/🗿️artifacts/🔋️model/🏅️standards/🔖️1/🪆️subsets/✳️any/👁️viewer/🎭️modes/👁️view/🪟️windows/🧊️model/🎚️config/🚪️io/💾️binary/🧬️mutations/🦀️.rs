//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::modes::view::windows::model::config::mutations::*;
use crate::modes::view::windows::model::config::{EnergyModelViewerCameraPose, EnergyModelViewerWindowConfig};

dsl::original_pack_value_operation_receiving!(EnergyModelViewerWindowConfigMutation);

impl protocol::OpBinary for EnergyModelViewerWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = protocol::os_pack::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })
    }
}
}
