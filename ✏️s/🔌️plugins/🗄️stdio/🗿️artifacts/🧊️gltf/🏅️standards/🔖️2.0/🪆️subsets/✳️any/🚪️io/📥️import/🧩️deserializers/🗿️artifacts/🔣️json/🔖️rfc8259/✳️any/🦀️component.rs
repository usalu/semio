//! 📥️ Deserialize `stdio.gltf` from stdio.json.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::json::JsonSnapshot;
pub async fn register() {}
/// 🧪️ P2-FG3: real cross-artifact glue — was a literal `serde_json::to_vec(&from.value)` JSON
/// round-trip (flagged by `p2-w0-recon-report.md`'s own JSON-transfer census as a real transfer-
/// path violation, "in scope for this program... FG3, gltf row"). Reuses json's OWN real
/// hand-rolled `write_json_text` (`🔣️json/…/📸️snapshot/🦀️component.rs`, no `serde_json` involved
/// — the SAME text codec json's own `ArtifactPack::encode_pack_with` uses for its native format)
/// to turn the already-parsed `JsonValue` tree back into real JSON text bytes, which is exactly
/// what `parse_gltf_document` (glTF's own real `.gltf` JSON text codec) expects — no
/// `serde_json::Value` anywhere on this transfer path anymore.
pub async fn deserialize(from: &JsonSnapshot) -> Result<GltfSnapshot, store::TextError> {
    let text = crate::artifacts::json::schema::snapshot::write_json_text(&from.value).await;
    crate::artifacts::gltf::engine::parse_gltf_document(text.as_bytes()).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
pub async fn deserialize_text(text: &str) -> Result<GltfSnapshot, store::TextError> {
    deserialize(&<JsonSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await
}
