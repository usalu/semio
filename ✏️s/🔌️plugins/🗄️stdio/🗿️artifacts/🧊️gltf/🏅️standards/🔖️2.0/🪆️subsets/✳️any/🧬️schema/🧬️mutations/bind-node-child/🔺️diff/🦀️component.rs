//! 🔺️ Minimal deterministic child-edge insertion patch.
//!
use crate::artifacts::gltf::schema::mutations::bind_node_child::mutation::{validate, GltfBindNodeChildPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindNodeChildDiff {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
    pub touched_paths: Vec<String>,
}
pub async fn derive(payload: &GltfBindNodeChildPayload, base: &GltfSnapshot) -> Result<GltfBindNodeChildDiff, GltfTopLevelMutationRejection> {
    validate(payload, base).await?;
    Ok(GltfBindNodeChildDiff { parent: payload.parent, child: payload.child, position: payload.position, touched_paths: vec![format!("document/nodes/{}/children/{}", payload.parent, payload.position)] })
}
pub async fn apply(base: &GltfSnapshot, diff: &GltfBindNodeChildDiff) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/nodes/{}/children/{}", diff.parent, diff.position);
    if diff.touched_paths.len() != 1 || diff.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "patch touched path does not match its edge coordinates").await);
    }
    let parent = base.document.nodes.get(diff.parent).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/nodes", "parent is absent"))?;
    if base.document.nodes.get(diff.child).is_none() || diff.position > parent.children.len() || parent.children.contains(&diff.child) {
        return Err(reject("gltf.mutation.stale-diff", format!("document/nodes/{}/children/{}", diff.parent, diff.position), "parent, position, or child binding is stale").await);
    }
    let mut pending = vec![diff.child];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(node) = pending.pop() {
        if node == diff.parent {
            return Err(reject("gltf.mutation.node-cycle", format!("document/nodes/{}/children/{}", diff.parent, diff.position), "patch closes a cycle").await);
        }
        if seen.insert(node) {
            let current = base.document.nodes.get(node).ok_or_else(|| reject("gltf.mutation.invalid-reference", format!("document/nodes/{}", node), "child graph contains a missing node"))?;
            pending.extend(current.children.iter().copied());
        }
    }
    let mut next = base.clone();
    next.document.nodes[diff.parent].children.insert(diff.position, diff.child);
    Ok(next)
}
pub async fn encode(diff: &GltfBindNodeChildDiff) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(diff)
}
