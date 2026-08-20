//! 🦀 move-node-child: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.move-node-child.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMoveNodeChildPayload { pub parent: usize, pub child: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveNodeChildPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?; let children = &base.document.nodes[payload.parent].children; let index = children.iter().position(|child| *child == payload.child).ok_or_else(|| reject("gltf.mutation.relation-absent", "document/nodes/children", "child is not linked to parent"))?; checked_index(payload.position, children.len(), "document/nodes/children")?; if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/nodes/children", "destination equals source")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveNodeChildPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let children = &mut next.document.nodes[payload.parent].children; let index = children.iter().position(|child| *child == payload.child).expect("validated child"); let child = children.remove(index); children.insert(payload.position, child); Ok(next) }
