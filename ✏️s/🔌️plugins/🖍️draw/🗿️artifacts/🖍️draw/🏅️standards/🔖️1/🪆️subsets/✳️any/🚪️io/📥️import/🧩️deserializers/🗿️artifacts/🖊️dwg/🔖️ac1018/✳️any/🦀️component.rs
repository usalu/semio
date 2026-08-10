//! draw <- dwg
use crate::artifacts::draw::DrawSnapshot;
use semio_framework::{dwg_from_bytes, DwgDrawing};
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;
pub fn register() {}
pub fn deserialize(from: &DwgSnapshot) -> Result<DrawSnapshot, String> { deserialize_bytes(&from.bytes) }
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DrawSnapshot, String> {
    let _meta = decode_dwg(bytes)?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes)?;
    let value = crate::artifacts::draw::engine::draw_document_json_from_dwg(&drawing)?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}
