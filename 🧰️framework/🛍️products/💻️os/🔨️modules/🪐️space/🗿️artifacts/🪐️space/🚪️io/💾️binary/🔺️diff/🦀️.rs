//! 🚪️ Artifact diff representation.

#[allow(unused_imports)]
mod diff_codec {
use crate::*;
use protocol::{DiffText,DiffBinary};
use io::sqlite::snapshot::{register_sqlite_snapshot,SQLITE_SNAPSHOT_DIALECT};
use serde::{Deserialize, Serialize};

impl DiffBinary for crate::SpaceDiff {
    fn encode_diff(&self) -> Result<Vec<u8>, semio_framework_os_kernel::ProtocolError> {
        Ok(semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, semio_framework_os_kernel::ProtocolError> {
        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "space-diff", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| semio_framework_os_kernel::ProtocolError::Malformed { what: "space-diff", offset: 0, detail: error.to_string() })
    }
}
}
pub use diff_codec::*;
