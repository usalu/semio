//! 🦀 bind-node-child: typed validation and atomic application.
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.bind-node-child.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindNodeChildPayload {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
}

pub async fn validate(payload: &GltfBindNodeChildPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?;
    checked_index(payload.child, base.document.nodes.len(), "document/nodes")?;
    checked_position(payload.position, base.document.nodes[payload.parent].children.len(), "document/nodes/children")?;
    if payload.parent == payload.child || base.document.nodes[payload.parent].children.contains(&payload.child) {
        return Err(reject("gltf.mutation.invalid-child-link", format!("document/nodes/{}/children", payload.parent), "self and duplicate child links are forbidden"));
    }
    let mut pending = vec![payload.child];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(node) = pending.pop() {
        if node == payload.parent {
            return Err(reject("gltf.mutation.node-cycle", format!("document/nodes/{}/children", payload.parent), "relationship closes a cycle"));
        }
        if seen.insert(node) {
            let current = base.document.nodes.get(node).ok_or_else(|| reject("gltf.mutation.invalid-reference", format!("document/nodes/{}", node), "child graph contains a missing node"))?;
            pending.extend(current.children.iter().copied());
        }
    }
    Ok(())
}

pub async fn apply(payload: &GltfBindNodeChildPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let mut next = base.clone();
    next.document.nodes[payload.parent].children.insert(payload.position, payload.child);
    Ok(next)
}
