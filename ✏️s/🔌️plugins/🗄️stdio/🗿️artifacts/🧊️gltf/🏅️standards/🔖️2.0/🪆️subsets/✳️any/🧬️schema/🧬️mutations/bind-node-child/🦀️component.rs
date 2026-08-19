//! 🧬️ Executable descriptor adapter for bind-node-child.
use crate::artifacts::gltf::schema::mutations::bind_node_child::{diff, inverse, mutation};
use crate::artifacts::gltf::schema::mutations::{GltfMutationLeafApplication, GltfMutationLeafDescriptor, GltfMutationLeafError, GltfMutationLeafPlan};
use crate::artifacts::gltf::GltfSnapshot;
use serde::de::DeserializeOwned;
use serde::Serialize;

async fn rejection(error: crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection) -> GltfMutationLeafError {
    GltfMutationLeafError { code: error.code, path: error.path, detail: error.detail }
}
async fn payload_error(detail: impl ToString) -> GltfMutationLeafError {
    GltfMutationLeafError { code: "gltf.mutation.invalid-payload".into(), path: "payload".into(), detail: detail.to_string() }
}
async fn encode_error(detail: impl ToString) -> GltfMutationLeafError {
    GltfMutationLeafError { code: "gltf.mutation.encode-failed".into(), path: "payload".into(), detail: detail.to_string() }
}
async fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, GltfMutationLeafError> {
    serde_json::from_slice(bytes).map_err(payload_error)
}
async fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, GltfMutationLeafError> {
    serde_json::to_vec(value).map_err(encode_error)
}
async fn path(parent: usize, position: usize) -> String {
    format!("document/nodes/{}/children/{}", parent, position)
}
async fn plan(bytes: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let payload: mutation::GltfBindNodeChildPayload = decode(bytes)?;
    let diff = diff::derive(&payload, base).map_err(rejection)?;
    let inverse = inverse::derive(&payload, base).map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&diff)?, inverse_payload: encode(&inverse)?, touched_paths: vec![path(diff.parent, diff.position)] })
}
async fn plan_inverse(bytes: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let inverse: inverse::GltfBindNodeChildInverse = decode(bytes)?;
    let _ = inverse::apply(base, &inverse).map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&inverse)?, inverse_payload: Vec::new(), touched_paths: vec![path(inverse.parent, inverse.position)] })
}
async fn apply_diff(bytes: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let diff: diff::GltfBindNodeChildDiff = decode(bytes)?;
    let touched_paths = vec![path(diff.parent, diff.position)];
    let snapshot = diff::apply(base, &diff).map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}
async fn apply_inverse(bytes: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let inverse: inverse::GltfBindNodeChildInverse = decode(bytes)?;
    let touched_paths = vec![path(inverse.parent, inverse.position)];
    let snapshot = inverse::apply(base, &inverse).map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}
pub const DESCRIPTOR: GltfMutationLeafDescriptor = GltfMutationLeafDescriptor { command_id: mutation::ID, version: 1, plan, plan_inverse, apply_diff, apply_inverse };
