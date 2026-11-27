//! 🧾️ Semio Pack representation for PDF 1.4 mutations.

use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
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

impl store::ArtifactPersistedOperation for PdfMutation {
    /// 🎒️ Persisted PDF 1.4 operations are Pack Values whose conflict segments the document's own Store catalog and its replacements share.
    fn install_persisted_receiving(wire: store::ArtifactOperationWire<Self>) -> store::ArtifactOperationWire<Self> {
        wire.with_original_pack_value_receiving().with_original_target(Self::original_conflict_target)
    }
}
