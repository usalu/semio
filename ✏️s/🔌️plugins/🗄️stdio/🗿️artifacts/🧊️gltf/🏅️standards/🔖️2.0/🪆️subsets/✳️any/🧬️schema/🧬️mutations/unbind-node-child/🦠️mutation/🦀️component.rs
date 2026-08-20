//! 🦀 unbind-node-child: typed validation and atomic application.
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::checked_index;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.unbind-node-child.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfUnbindNodeChildPayload {
    pub parent: usize,
    pub child: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfUnbindNodeChildPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?;
    checked_index(payload.child, base.document.nodes.len(), "document/nodes")?;
    if !base.document.nodes[payload.parent].children.contains(&payload.child) {
        return Err(reject("gltf.mutation.relation-absent", format!("document/nodes/{}/children", payload.parent), "child is not linked to parent"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfUnbindNodeChildPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let mut next = base.clone();
    let position =
        next.document.nodes[payload.parent].children.iter().position(|child| *child == payload.child).ok_or_else(|| reject("gltf.mutation.relation-absent", format!("document/nodes/{}/children", payload.parent), "child is not linked to parent"))?;
    next.document.nodes[payload.parent].children.remove(position);
    Ok(next)
}
