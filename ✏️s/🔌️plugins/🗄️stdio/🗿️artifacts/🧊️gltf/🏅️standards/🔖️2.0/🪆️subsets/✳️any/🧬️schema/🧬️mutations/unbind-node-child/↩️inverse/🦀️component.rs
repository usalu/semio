//! ↩️ Exact child-edge reinsertion inverse.
//!
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::mutations::unbind_node_child::mutation::{validate, GltfUnbindNodeChildPayload};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfUnbindNodeChildInverse {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
    pub expected_children: Vec<usize>,
    pub touched_paths: Vec<String>,
}

pub fn derive(payload: &GltfUnbindNodeChildPayload, base: &GltfSnapshot) -> Result<GltfUnbindNodeChildInverse, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let position =
        base.document.nodes[payload.parent].children.iter().position(|child| *child == payload.child).ok_or_else(|| reject("gltf.mutation.relation-absent", format!("document/nodes/{}/children", payload.parent), "child is not linked to parent"))?;
    let after = crate::artifacts::gltf::schema::mutations::unbind_node_child::mutation::apply(payload, base)?;
    Ok(GltfUnbindNodeChildInverse {
        parent: payload.parent,
        child: payload.child,
        position,
        expected_children: after.document.nodes[payload.parent].children.clone(),
        touched_paths: vec![format!("document/nodes/{}/children/{}", payload.parent, position)],
    })
}

pub fn apply(base: &GltfSnapshot, inverse: &GltfUnbindNodeChildInverse) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/nodes/{}/children/{}", inverse.parent, inverse.position);
    if inverse.touched_paths.len() != 1 || inverse.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "inverse touched path does not match its edge coordinates"));
    }
    let children = &base.document.nodes.get(inverse.parent).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "parent is absent"))?.children;
    if *children != inverse.expected_children || inverse.position > children.len() || children.contains(&inverse.child) {
        return Err(reject("gltf.mutation.stale-inverse", format!("document/nodes/{}/children/{}", inverse.parent, inverse.position), "children do not equal the planned forward state"));
    }
    let mut next = base.clone();
    next.document.nodes[inverse.parent].children.insert(inverse.position, inverse.child);
    Ok(next)
}

pub fn encode(inverse: &GltfUnbindNodeChildInverse) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(inverse)
}
