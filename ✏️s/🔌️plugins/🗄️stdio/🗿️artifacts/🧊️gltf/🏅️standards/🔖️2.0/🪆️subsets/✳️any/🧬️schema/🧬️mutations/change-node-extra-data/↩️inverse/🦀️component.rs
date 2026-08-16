//! ↩️ Exact node extras restoration inverse.
use crate::artifacts::gltf::schema::mutations::change_node_extra_data::mutation::{validate, GltfChangeNodeExtraDataPayload, GltfDataPresence};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::snapshot::GltfJson;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeExtraDataInverse { pub node: usize, pub before: GltfDataPresence, pub after: GltfDataPresence, pub touched_paths: Vec<String> }
fn presence(value: &Option<GltfJson>) -> GltfDataPresence { match value { Some(value) => GltfDataPresence::Present { value: value.clone() }, None => GltfDataPresence::Absent } }
fn value(data: &GltfDataPresence) -> Option<GltfJson> { match data { GltfDataPresence::Absent => None, GltfDataPresence::Present { value } => Some(value.clone()) } }
pub fn derive(payload: &GltfChangeNodeExtraDataPayload, base: &GltfSnapshot) -> Result<GltfChangeNodeExtraDataInverse, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfChangeNodeExtraDataInverse { node: payload.node, before: presence(&base.document.nodes[payload.node].extras), after: payload.data.clone(), touched_paths: vec![format!("document/nodes/{}/extras", payload.node)] }) }
pub fn apply(base: &GltfSnapshot, inverse: &GltfChangeNodeExtraDataInverse) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { let path = format!("document/nodes/{}/extras", inverse.node); if inverse.touched_paths.len() != 1 || inverse.touched_paths[0] != path { return Err(reject("gltf.mutation.invalid-touched-path", path, "serialized touched paths must equal the exact node-extras path")); } let node = base.document.nodes.get(inverse.node).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "node is absent"))?; if presence(&node.extras) != inverse.after { return Err(reject("gltf.mutation.stale-inverse", format!("document/nodes/{}/extras", inverse.node), "node or expected changed extras is stale")); } let mut next = base.clone(); next.document.nodes[inverse.node].extras = value(&inverse.before); Ok(next) }
pub fn encode(inverse: &GltfChangeNodeExtraDataInverse) -> Result<Vec<u8>, serde_json::Error> { serde_json::to_vec(inverse) }
