//! 📥️ Deserialize `stdio.gltf` from stdio.json.
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::gltf::GltfSnapshot;
pub fn register() {}
pub fn deserialize(from: &JsonSnapshot) -> Result<GltfSnapshot, store::TextError> {
    let bytes = serde_json::to_vec(&from.value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    crate::artifacts::gltf::engine::parse_gltf_document(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
pub fn deserialize_text(text: &str) -> Result<GltfSnapshot, store::TextError> {
    deserialize(&<JsonSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
