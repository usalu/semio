//! jack <- svg
use crate::artifacts::jack::JackSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &SvgSnapshot) -> Result<JackSnapshot, store::TextError> {
    let _ = (STDIO_SVG_DOCUMENT_SCHEMA, from);
    Ok(JackSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<JackSnapshot, store::TextError> {
    let _ = bytes;
    Ok(JackSnapshot::default())
}
