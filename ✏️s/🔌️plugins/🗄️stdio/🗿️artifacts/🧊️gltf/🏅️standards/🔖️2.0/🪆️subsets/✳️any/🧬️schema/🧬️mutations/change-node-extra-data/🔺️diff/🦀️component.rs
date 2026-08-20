//! 🔺️ Exact node extras replacement patch.
use crate::artifacts::gltf::schema::mutations::change_node_extra_data::mutation::{validate, GltfChangeNodeExtraDataPayload, GltfDataPresence};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::snapshot::GltfJson;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeExtraDataDiff { pub node: usize, pub before: GltfDataPresence, pub after: GltfDataPresence, pub touched_paths: Vec<String> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn presence(value: &Option<GltfJson>) -> GltfDataPresence { match value { Some(value) => GltfDataPresence::Present { value: value.clone() }, None => GltfDataPresence::Absent } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value(data: &GltfDataPresence) -> Option<GltfJson> { match data { GltfDataPresence::Absent => None, GltfDataPresence::Present { value } => Some(value.clone()) } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfChangeNodeExtraDataPayload, base: &GltfSnapshot) -> Result<GltfChangeNodeExtraDataDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfChangeNodeExtraDataDiff { node: payload.node, before: presence(&base.document.nodes[payload.node].extras), after: payload.data.clone(), touched_paths: vec![format!("document/nodes/{}/extras", payload.node)] }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(base: &GltfSnapshot, diff: &GltfChangeNodeExtraDataDiff) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { let path = format!("document/nodes/{}/extras", diff.node); if diff.touched_paths.len() != 1 || diff.touched_paths[0] != path { return Err(reject("gltf.mutation.invalid-touched-path", path, "serialized touched paths must equal the exact node-extras path")); } let node = base.document.nodes.get(diff.node).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "node is absent"))?; if presence(&node.extras) != diff.before { return Err(reject("gltf.mutation.stale-diff", format!("document/nodes/{}/extras", diff.node), "node or expected previous extras is stale")); } let mut next = base.clone(); next.document.nodes[diff.node].extras = value(&diff.after); Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfChangeNodeExtraDataDiff) -> Result<Vec<u8>, serde_json::Error> { serde_json::to_vec(diff) }
