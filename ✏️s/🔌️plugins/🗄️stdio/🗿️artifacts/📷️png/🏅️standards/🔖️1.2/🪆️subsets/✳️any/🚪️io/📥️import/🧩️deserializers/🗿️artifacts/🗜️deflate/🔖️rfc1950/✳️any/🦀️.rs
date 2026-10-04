//! Deserialize stdio.png from stdio.deflate (raw file bytes in deflate snapshot).

use crate::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_deflate::DeflateSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &DeflateSnapshot) -> Result<PngSnapshot, store::PackError> {
    let mut snap = crate::engine::decode_png(&from.payload).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
    snap.schema = STDIO_PNG_DOCUMENT_SCHEMA.into();
    Ok(snap)
}
