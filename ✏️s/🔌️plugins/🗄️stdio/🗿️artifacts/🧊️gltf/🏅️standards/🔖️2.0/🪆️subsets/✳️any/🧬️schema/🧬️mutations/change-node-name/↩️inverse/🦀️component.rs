//! ↩️ Exact node-name restoration inverse.
use crate::artifacts::gltf::schema::mutations::change_node_name::mutation::{validate, GltfChangeNodeNamePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeNameInverse { pub node: usize, pub before: Option<String>, pub after: Option<String>, pub touched_paths: Vec<String> }
pub fn derive(payload: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<GltfChangeNodeNameInverse, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfChangeNodeNameInverse { node: payload.node, before: base.document.nodes[payload.node].name.clone(), after: payload.value.clone(), touched_paths: vec![format!("document/nodes/{}/name", payload.node)] }) }
pub fn apply(base: &GltfSnapshot, inverse: &GltfChangeNodeNameInverse) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { let path = format!("document/nodes/{}/name", inverse.node); if inverse.touched_paths.len() != 1 || inverse.touched_paths[0] != path { return Err(reject("gltf.mutation.invalid-touched-path", path, "serialized touched paths must equal the exact node-name path")); } let node = base.document.nodes.get(inverse.node).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "node is absent"))?; if node.name != inverse.after { return Err(reject("gltf.mutation.stale-inverse", format!("document/nodes/{}/name", inverse.node), "node or expected changed name is stale")); } let mut next = base.clone(); next.document.nodes[inverse.node].name = inverse.before.clone(); Ok(next) }
pub fn encode(inverse: &GltfChangeNodeNameInverse) -> Result<Vec<u8>, serde_json::Error> { serde_json::to_vec(inverse) }
