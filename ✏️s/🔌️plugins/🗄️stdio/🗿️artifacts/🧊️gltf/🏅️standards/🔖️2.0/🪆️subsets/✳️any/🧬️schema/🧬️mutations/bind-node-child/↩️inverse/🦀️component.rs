//! ↩️ Exact child-edge removal inverse.
//!
use crate::artifacts::gltf::schema::mutations::bind_node_child::mutation::{validate, GltfBindNodeChildPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindNodeChildInverse {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
    pub expected_children: Vec<usize>,
    pub touched_paths: Vec<String>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfBindNodeChildPayload, base: &GltfSnapshot) -> Result<GltfBindNodeChildInverse, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let after = crate::artifacts::gltf::schema::mutations::bind_node_child::mutation::apply(payload, base)?;
    Ok(GltfBindNodeChildInverse {
        parent: payload.parent,
        child: payload.child,
        position: payload.position,
        expected_children: after.document.nodes[payload.parent].children.clone(),
        touched_paths: vec![format!("document/nodes/{}/children/{}", payload.parent, payload.position)],
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(base: &GltfSnapshot, inverse: &GltfBindNodeChildInverse) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/nodes/{}/children/{}", inverse.parent, inverse.position);
    if inverse.touched_paths.len() != 1 || inverse.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "inverse touched path does not match its edge coordinates"));
    }
    let children = &base.document.nodes.get(inverse.parent).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "parent is absent"))?.children;
    if *children != inverse.expected_children || children.get(inverse.position) != Some(&inverse.child) {
        return Err(reject("gltf.mutation.stale-inverse", format!("document/nodes/{}/children/{}", inverse.parent, inverse.position), "children do not equal the planned forward state"));
    }
    let mut next = base.clone();
    next.document.nodes[inverse.parent].children.remove(inverse.position);
    Ok(next)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(inverse: &GltfBindNodeChildInverse) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(inverse)
}
