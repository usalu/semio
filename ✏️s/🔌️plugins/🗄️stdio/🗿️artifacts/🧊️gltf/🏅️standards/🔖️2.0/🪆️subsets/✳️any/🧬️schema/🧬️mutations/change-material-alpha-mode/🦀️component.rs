//! 🧩️ Executable descriptor for the change-material-alpha-mode leaf.

use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::{diff, inverse, mutation};
use crate::artifacts::gltf::schema::mutations::{GltfMutationLeafApplication, GltfMutationLeafDescriptor, GltfMutationLeafError, GltfMutationLeafPlan};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{de::DeserializeOwned, Serialize};

pub const DESCRIPTOR: GltfMutationLeafDescriptor = GltfMutationLeafDescriptor { command_id: mutation::ID, version: 1, plan, plan_inverse, apply_diff, apply_inverse };

async fn rejection(value: mutation::GltfChangeMaterialAlphaModeRejection) -> GltfMutationLeafError {
    GltfMutationLeafError { code: value.code, path: value.path, detail: value.detail }
}

async fn decode<T: DeserializeOwned>(payload: &[u8]) -> Result<T, GltfMutationLeafError> {
    serde_json::from_slice(payload).map_err(|error| GltfMutationLeafError { code: "gltf.mutation.malformed-payload".into(), path: "payload".into(), detail: error.to_string() })
}

async fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, GltfMutationLeafError> {
    serde_json::to_vec(value).map_err(|error| GltfMutationLeafError { code: "gltf.mutation.encode-failed".into(), path: "payload".into(), detail: error.to_string() })
}

async fn plan(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let payload = decode::<mutation::GltfChangeMaterialAlphaModePayload>(payload).await?;
    let diff = diff::derive(&payload, base).await.map_err(rejection)?;
    let inverse = inverse::reconstruct(&payload, base).await.map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&diff).await?, inverse_payload: encode(&inverse).await?, touched_paths: diff.expected_touched_paths().await })
}

async fn plan_inverse(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let inverse = decode::<inverse::GltfChangeMaterialAlphaModeInverse>(payload).await?;
    let mut candidate = base.clone();
    inverse.apply(&mut candidate).await.map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&inverse).await?, inverse_payload: Vec::new(), touched_paths: inverse.expected_touched_paths().await })
}

async fn apply_diff(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let diff = decode::<diff::GltfChangeMaterialAlphaModeDiff>(payload).await?;
    let touched_paths = diff.expected_touched_paths();
    let mut snapshot = base.clone();
    diff.apply(&mut snapshot).await.map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}

async fn apply_inverse(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let inverse = decode::<inverse::GltfChangeMaterialAlphaModeInverse>(payload).await?;
    let touched_paths = inverse.expected_touched_paths();
    let mut snapshot = base.clone();
    inverse.apply(&mut snapshot).await.map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}
