//! 🧾️ Semio Pack representation for Base XLSX mutations.

use crate::standards::v_ecma_376::subsets::base::schema::mutations::XlsxMutation;
use protocol::{OpBinary, ProtocolError};
use semio_framework_value::{FromValue, ToValue};
use store::os_pack::pack_rt::{decode_pack_value, encode_pack_value, renormalize_whole_number_floats};

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

impl OpBinary for XlsxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(encode_pack_value(&self.to_value()))
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let malformed = |detail| ProtocolError::Malformed { what: "XlsxMutation", offset: 0, detail };
        let value = decode_pack_value(bytes).map_err(|error| malformed(error.to_string()))?;
        Self::from_value(renormalize_whole_number_floats(value)).map_err(|error| malformed(error.to_string()))
    }
}
