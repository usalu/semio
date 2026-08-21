//! raster <- json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see w5b-close-report.md):
//! `JsonSnapshot::to_serde_value`/stdio's own real `parse_json_text` do the structural conversion —
//! no hand-rolled bridge needed here.
use crate::artifacts::raster::{RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, JsonSnapshot};
pub async fn register() {}

pub async fn deserialize(from: &JsonSnapshot) -> Result<RasterSnapshot, String> {
    let mut snap: RasterSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() {
        snap.schema = RASTER_DOCUMENT_SCHEMA.into();
    }
    Ok(snap)
}
pub async fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value = parse_json_text(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot::from_value(value))
}
