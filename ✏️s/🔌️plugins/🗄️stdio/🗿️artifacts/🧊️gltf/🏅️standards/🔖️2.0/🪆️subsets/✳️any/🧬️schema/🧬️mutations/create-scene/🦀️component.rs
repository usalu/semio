//! 🧩️ Common-descriptor adapter for `create-scene.v1`.

use crate::artifacts::gltf::schema::mutations::create_scene::private::GltfCreateSceneRejection;
use crate::artifacts::gltf::schema::mutations::create_scene::{diff, inverse, mutation};
use crate::artifacts::gltf::schema::mutations::{GltfMutationLeafApplication, GltfMutationLeafDescriptor, GltfMutationLeafError, GltfMutationLeafPlan};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{de::DeserializeOwned, Serialize};

pub const DESCRIPTOR: GltfMutationLeafDescriptor = GltfMutationLeafDescriptor { command_id: mutation::ID, version: 1, plan, plan_inverse, apply_diff, apply_inverse };

async fn rejection(value: GltfCreateSceneRejection) -> GltfMutationLeafError {
    GltfMutationLeafError { code: value.code, path: value.path, detail: value.detail }
}

async fn decode<T: DeserializeOwned>(payload: &[u8], path: &str) -> Result<T, GltfMutationLeafError> {
    serde_json::from_slice(payload).map_err(|error| GltfMutationLeafError { code: "gltf.mutation.malformed-payload".into(), path: path.into(), detail: error.to_string() })
}

async fn encode<T: Serialize>(value: &T, path: &str) -> Result<Vec<u8>, GltfMutationLeafError> {
    serde_json::to_vec(value).map_err(|error| GltfMutationLeafError { code: "gltf.mutation.encode-failed".into(), path: path.into(), detail: error.to_string() })
}

async fn plan(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let payload = decode::<mutation::GltfCreateScenePayload>(payload, "mutation/payload")?;
    mutation::validate(&payload, base).map_err(rejection)?;
    let forward = diff::derive(base, payload.position).map_err(rejection)?;
    let reverse = inverse::derive(base, payload.position).map_err(rejection)?;
    let touched_paths = diff::touched_paths(&forward, base).map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&forward, "diff")?, inverse_payload: encode(&reverse, "inverse")?, touched_paths })
}

async fn plan_inverse(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let inverse = decode::<inverse::GltfCreateSceneInverse>(payload, "inverse/payload")?;
    let touched_paths = inverse::touched_paths(&inverse, base).map_err(rejection)?;
    inverse::apply(&inverse, base).map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&inverse, "inverse")?, inverse_payload: Vec::new(), touched_paths })
}

async fn apply_diff(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let diff = decode::<diff::GltfCreateSceneDiff>(payload, "diff/payload")?;
    let snapshot = diff::apply(&diff, base).map_err(rejection)?;
    let touched_paths = diff::touched_paths(&diff, base).map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}

async fn apply_inverse(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let inverse = decode::<inverse::GltfCreateSceneInverse>(payload, "inverse/payload")?;
    let snapshot = inverse::apply(&inverse, base).map_err(rejection)?;
    let touched_paths = inverse::touched_paths(&inverse, base).map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}
