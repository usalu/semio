//! 🧩️ Leaf-owned executable adapter for `delete-scene.v1`.
use crate::artifacts::gltf::schema::mutations::delete_scene::{diff, inverse, mutation};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::de::DeserializeOwned;

pub struct GltfDeleteSceneLeafPlan {
    pub diff_payload: Vec<u8>,
    pub inverse_payload: Vec<u8>,
    pub touched_paths: Vec<String>,
}
pub struct GltfDeleteSceneDescriptorAdapter {
    pub command_id: &'static str,
    pub version: u32,
    pub derive: fn(&[u8], &GltfSnapshot) -> Result<GltfDeleteSceneLeafPlan, GltfTopLevelMutationRejection>,
    pub apply_diff: fn(&[u8], &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection>,
    pub apply_inverse: fn(&[u8], &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection>,
    pub inspect_diff: fn(&[u8]) -> Result<Vec<String>, GltfTopLevelMutationRejection>,
    pub inspect_inverse: fn(&[u8]) -> Result<Vec<String>, GltfTopLevelMutationRejection>,
}
pub const DESCRIPTOR: GltfDeleteSceneDescriptorAdapter = GltfDeleteSceneDescriptorAdapter { command_id: mutation::ID, version: 1, derive, apply_diff, apply_inverse, inspect_diff, inspect_inverse };
fn decode<T: DeserializeOwned>(bytes: &[u8], path: &str) -> Result<T, GltfTopLevelMutationRejection> {
    serde_json::from_slice(bytes).map_err(|error| reject("gltf.mutation.decode-failed", path, error.to_string()))
}
pub fn derive(bytes: &[u8], base: &GltfSnapshot) -> Result<GltfDeleteSceneLeafPlan, GltfTopLevelMutationRejection> {
    let payload = decode::<mutation::GltfDeleteScenePayload>(bytes, "payload")?;
    let forward = diff::derive(base, payload.index)?;
    let reverse = inverse::derive(base, payload.index)?;
    Ok(GltfDeleteSceneLeafPlan { diff_payload: diff::encode(&forward)?, inverse_payload: inverse::encode(&reverse)?, touched_paths: forward.touched_paths })
}
pub fn apply_diff(bytes: &[u8], base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    diff::apply(&decode::<diff::GltfDeleteSceneDiff>(bytes, "diff")?, base)
}
pub fn apply_inverse(bytes: &[u8], after: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    inverse::apply(&decode::<inverse::GltfDeleteSceneInverse>(bytes, "inverse")?, after)
}
pub fn inspect_diff(bytes: &[u8]) -> Result<Vec<String>, GltfTopLevelMutationRejection> {
    Ok(decode::<diff::GltfDeleteSceneDiff>(bytes, "diff")?.touched_paths)
}
pub fn inspect_inverse(bytes: &[u8]) -> Result<Vec<String>, GltfTopLevelMutationRejection> {
    Ok(decode::<inverse::GltfDeleteSceneInverse>(bytes, "inverse")?.touched_paths)
}
