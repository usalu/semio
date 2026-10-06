//! 📥️ Deserialize `stdio.dwg` from stdio.binary.

use crate::DwgSnapshot;
use semio_s_artifact_stdio_binary::BinarySnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<DwgSnapshot, store::PackError> {
    crate::standards::v_ac1024::subsets::any::io::binary::snapshot::decode_dwg(&from.bytes).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DwgSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
