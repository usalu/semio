//! Deserialize stdio.gif from stdio.binary.

use semio_s_artifact_stdio_binary::BinarySnapshot;
use crate::standards::v89a::subsets::any::schema::snapshot::{GifSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<GifSnapshot, store::PackError> {
    let mut snap = crate::standards::v89a::engine::decode_gif(&from.bytes).map_err(store::PackError::Schema)?;
    snap.schema = STDIO_GIF89A_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<GifSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
