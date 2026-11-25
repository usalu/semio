//! 🧾️ Semio Pack representation for PDF 1.7 mutations.

use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use protocol::{OpBinary, ProtocolError};
use semio_framework_value::{FromValue, ToValue};
use store::os_pack::pack_rt::{decode_pack_value, encode_pack_value, renormalize_whole_number_floats};

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(encode_pack_value(&self.to_value()))
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let malformed = |detail| ProtocolError::Malformed { what: "PdfMutation", offset: 0, detail };
        let value = decode_pack_value(bytes).map_err(|error| malformed(error.to_string()))?;
        Self::from_value(renormalize_whole_number_floats(value)).map_err(|error| malformed(error.to_string()))
    }
}

dsl::original_pack_value_operation_receiving!(PdfMutation);
dsl::artifact_persisted_binary_operation!(PdfMutation);
