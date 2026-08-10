//! 📥️ Deserialize `stdio.gltf` from stdio.json.
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &JsonSnapshot) -> Result<GltfSnapshot, store::TextError> {
    let vertices = crate::artifacts::gltf::schema::snapshot::gltf_vertices_from_value(&from.value).map_err(|e| {
        store::TextError::new(e, dsl::TextSpan::at(1, 1))
    })?;
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), vertices, document: from.value.clone() })
}
pub fn deserialize_text(text: &str) -> Result<GltfSnapshot, store::TextError> {
    deserialize(&<JsonSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
