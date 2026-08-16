//! 🔺️ Exact child-edge removal patch.
//!
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::mutations::unbind_node_child::mutation::{validate, GltfUnbindNodeChildPayload};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfUnbindNodeChildDiff {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
    pub touched_paths: Vec<String>,
}

pub fn derive(payload: &GltfUnbindNodeChildPayload, base: &GltfSnapshot) -> Result<GltfUnbindNodeChildDiff, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let position =
        base.document.nodes[payload.parent].children.iter().position(|child| *child == payload.child).ok_or_else(|| reject("gltf.mutation.relation-absent", format!("document/nodes/{}/children", payload.parent), "child is not linked to parent"))?;
    Ok(GltfUnbindNodeChildDiff { parent: payload.parent, child: payload.child, position, touched_paths: vec![format!("document/nodes/{}/children/{}", payload.parent, position)] })
}

pub fn apply(base: &GltfSnapshot, diff: &GltfUnbindNodeChildDiff) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/nodes/{}/children/{}", diff.parent, diff.position);
    if diff.touched_paths.len() != 1 || diff.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "patch touched path does not match its edge coordinates"));
    }
    let children = &base.document.nodes.get(diff.parent).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "parent is absent"))?.children;
    if children.get(diff.position) != Some(&diff.child) {
        return Err(reject("gltf.mutation.stale-diff", format!("document/nodes/{}/children/{}", diff.parent, diff.position), "child is not at the recorded removal position"));
    }
    let mut next = base.clone();
    next.document.nodes[diff.parent].children.remove(diff.position);
    Ok(next)
}

pub fn encode(diff: &GltfUnbindNodeChildDiff) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(diff)
}
