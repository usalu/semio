//! 🦀 reparent-node: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.reparent-node.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReparentNodePayload { pub parent: usize, pub child: usize, pub position: usize }
pub async fn validate(payload: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?; checked_index(payload.child, base.document.nodes.len(), "document/nodes")?; if payload.parent == payload.child { return Err(reject("gltf.mutation.node-cycle", "document/nodes", "a node cannot parent itself")); } let mut pending = vec![payload.child]; let mut seen = std::collections::BTreeSet::new(); while let Some(node) = pending.pop() { if node == payload.parent { return Err(reject("gltf.mutation.node-cycle", "document/nodes", "relationship closes a cycle")); } if seen.insert(node) { pending.extend(base.document.nodes[node].children.iter().copied()); } } let length = base.document.nodes[payload.parent].children.iter().filter(|child| **child != payload.child).count(); checked_position(payload.position, length, "document/nodes/children")?; Ok(()) }
pub async fn apply(payload: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); for node in &mut next.document.nodes { node.children.retain(|child| *child != payload.child); } for scene in &mut next.document.scenes { scene.nodes.retain(|node| *node != payload.child); } next.document.nodes[payload.parent].children.insert(payload.position, payload.child); Ok(next) }
