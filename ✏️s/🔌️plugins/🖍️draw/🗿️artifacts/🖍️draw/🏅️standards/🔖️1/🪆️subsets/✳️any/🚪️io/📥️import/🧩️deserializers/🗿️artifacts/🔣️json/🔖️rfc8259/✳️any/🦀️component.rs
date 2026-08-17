//! draw <- json
//!
//! 🔁️ Bridges via stdio's own real RFC8259 text codec (`parse_json_text`/`write_json_text`), not
//! `serde_json::Value`, since `JsonSnapshot.value` is stdio's own lexeme-preserving `JsonValue`
//! model now (ticket 26/08/11's object-subset rework landed in stdio; this leaf's own call site
//! was lagging — see `w5b-w-report.md` for the full note).
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;
pub fn register() {}
pub fn deserialize(from: &JsonSnapshot) -> Result<DrawSnapshot, String> {
    let mut snap: DrawSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() { snap.schema = DRAW_DOCUMENT_SCHEMA.into(); }
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DrawSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value = parse_json_text(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot::from_value(value))
}
