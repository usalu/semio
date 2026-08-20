//! ↩️ reparent-node: direct sparse undo derivation limited to owned collections.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfScenesDiff, GltfNodesDiff, GltfMeshesDiff, GltfAccessorsDiff, GltfBufferViewsDiff, GltfBuffersDiff, GltfBufferBytesDiff};
use crate::artifacts::gltf::schema::mutations::reparent_node::mutation::{apply, GltfReparentNodePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(GltfDiff { scenes: Some(GltfScenesDiff::between(&next.document.scenes, &base.document.scenes)), nodes: Some(GltfNodesDiff::between(&next.document.nodes, &base.document.nodes)), ..Default::default() }) }
