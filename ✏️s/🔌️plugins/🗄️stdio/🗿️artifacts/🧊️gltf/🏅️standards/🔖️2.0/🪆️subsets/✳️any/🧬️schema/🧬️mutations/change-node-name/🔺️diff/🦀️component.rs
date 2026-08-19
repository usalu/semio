//! 🔺️ Exact node-name replacement patch.
use crate::artifacts::gltf::schema::mutations::change_node_name::mutation::{validate, GltfChangeNodeNamePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeNameDiff { pub node: usize, pub before: Option<String>, pub after: Option<String>, pub touched_paths: Vec<String> }
pub async fn derive(payload: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<GltfChangeNodeNameDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfChangeNodeNameDiff { node: payload.node, before: base.document.nodes[payload.node].name.clone(), after: payload.value.clone(), touched_paths: vec![format!("document/nodes/{}/name", payload.node)] }) }
pub async fn apply(base: &GltfSnapshot, diff: &GltfChangeNodeNameDiff) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { let path = format!("document/nodes/{}/name", diff.node); if diff.touched_paths.len() != 1 || diff.touched_paths[0] != path { return Err(reject("gltf.mutation.invalid-touched-path", path, "serialized touched paths must equal the exact node-name path")); } let node = base.document.nodes.get(diff.node).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "node is absent"))?; if node.name != diff.before { return Err(reject("gltf.mutation.stale-diff", format!("document/nodes/{}/name", diff.node), "node or expected previous name is stale")); } let mut next = base.clone(); next.document.nodes[diff.node].name = diff.after.clone(); Ok(next) }
pub async fn encode(diff: &GltfChangeNodeNameDiff) -> Result<Vec<u8>, serde_json::Error> { serde_json::to_vec(diff) }
