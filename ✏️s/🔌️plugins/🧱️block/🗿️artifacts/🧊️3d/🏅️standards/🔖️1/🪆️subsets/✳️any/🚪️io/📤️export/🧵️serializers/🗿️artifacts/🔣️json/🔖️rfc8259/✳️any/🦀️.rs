//! block3d -> json
use crate::artifacts::block3d::Block3dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &Block3dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = dsl::ToValue::to_value(snapshot);
    Ok(JsonSnapshot::from_value(dsl::json::from_dsl_value(&raw)))
}

pub async fn serialize_bytes(snapshot: &Block3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_text(&serialize(snapshot)?.value).into_bytes())
}
