//! 📤️ Serialize `stdio.gltf` to stdio.json.
use crate::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::artifacts::gltf::GltfSnapshot;
pub fn register() {}
pub fn serialize(from: &GltfSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let value = if from.document.is_null() {
        crate::artifacts::gltf::schema::snapshot::gltf_value_from_vertices(&from.vertices)
    } else {
        from.document.clone()
    };
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
