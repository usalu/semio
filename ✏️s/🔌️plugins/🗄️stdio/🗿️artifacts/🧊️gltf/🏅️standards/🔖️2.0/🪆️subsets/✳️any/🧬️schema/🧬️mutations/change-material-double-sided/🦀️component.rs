//! 🧩️ Executable descriptor for the change-material-double-sided leaf.

use crate::artifacts::gltf::schema::mutations::change_material_double_sided::{diff, inverse, mutation};
use crate::artifacts::gltf::schema::mutations::{GltfMutationLeafApplication, GltfMutationLeafDescriptor, GltfMutationLeafError, GltfMutationLeafPlan};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{de::DeserializeOwned, Serialize};

pub const DESCRIPTOR: GltfMutationLeafDescriptor = GltfMutationLeafDescriptor { command_id: mutation::ID, version: 1, plan, plan_inverse, apply_diff, apply_inverse };

fn rejection(value: mutation::GltfChangeMaterialDoubleSidedRejection) -> GltfMutationLeafError {
    GltfMutationLeafError { code: value.code, path: value.path, detail: value.detail }
}
fn decode<T: DeserializeOwned>(payload: &[u8]) -> Result<T, GltfMutationLeafError> {
    serde_json::from_slice(payload).map_err(|error| GltfMutationLeafError { code: "gltf.mutation.malformed-payload".into(), path: "payload".into(), detail: error.to_string() })
}
fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, GltfMutationLeafError> {
    serde_json::to_vec(value).map_err(|error| GltfMutationLeafError { code: "gltf.mutation.encode-failed".into(), path: "payload".into(), detail: error.to_string() })
}
fn plan(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let payload = decode::<mutation::GltfChangeMaterialDoubleSidedPayload>(payload)?;
    let diff = diff::derive(&payload, base).map_err(rejection)?;
    let inverse = inverse::reconstruct(&payload, base).map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&diff)?, inverse_payload: encode(&inverse)?, touched_paths: diff.expected_touched_paths() })
}
fn plan_inverse(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError> {
    let inverse = decode::<inverse::GltfChangeMaterialDoubleSidedInverse>(payload)?;
    let mut candidate = base.clone();
    inverse.apply(&mut candidate).map_err(rejection)?;
    Ok(GltfMutationLeafPlan { diff_payload: encode(&inverse)?, inverse_payload: Vec::new(), touched_paths: inverse.expected_touched_paths() })
}
fn apply_diff(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let diff = decode::<diff::GltfChangeMaterialDoubleSidedDiff>(payload)?;
    let touched_paths = diff.expected_touched_paths();
    let mut snapshot = base.clone();
    diff.apply(&mut snapshot).map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}
fn apply_inverse(payload: &[u8], base: &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError> {
    let inverse = decode::<inverse::GltfChangeMaterialDoubleSidedInverse>(payload)?;
    let touched_paths = inverse.expected_touched_paths();
    let mut snapshot = base.clone();
    inverse.apply(&mut snapshot).map_err(rejection)?;
    Ok(GltfMutationLeafApplication { snapshot, touched_paths })
}
