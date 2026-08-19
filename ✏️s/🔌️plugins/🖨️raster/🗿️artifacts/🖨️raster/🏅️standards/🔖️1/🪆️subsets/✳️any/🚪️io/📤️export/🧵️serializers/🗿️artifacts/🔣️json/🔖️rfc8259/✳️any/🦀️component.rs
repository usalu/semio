//! raster -> json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see w5b-close-report.md):
//! `JsonSnapshot::from_value`/stdio's own real `write_json_pretty` do the structural conversion —
//! no hand-rolled bridge needed here.
use crate::artifacts::raster::RasterSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonSnapshot};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;
pub async fn register() {}

pub async fn serialize(snapshot: &RasterSnapshot) -> Result<JsonSnapshot, String> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| e.to_string())?;
    Ok(JsonSnapshot::from_value(value))
}
pub async fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
