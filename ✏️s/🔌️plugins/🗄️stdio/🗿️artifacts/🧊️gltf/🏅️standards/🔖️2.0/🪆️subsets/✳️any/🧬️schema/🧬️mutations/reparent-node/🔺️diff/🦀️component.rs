//! 🔺️ reparent-node: direct sparse forward derivation limited to owned collections.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfScenesDiff, GltfNodesDiff, GltfMeshesDiff, GltfAccessorsDiff, GltfBufferViewsDiff, GltfBuffersDiff, GltfBufferBytesDiff};
use crate::artifacts::gltf::schema::mutations::reparent_node::mutation::{apply, GltfReparentNodePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(GltfDiff { scenes: Some(GltfScenesDiff::between(&base.document.scenes, &next.document.scenes)), nodes: Some(GltfNodesDiff::between(&base.document.nodes, &next.document.nodes)), ..Default::default() }) }
