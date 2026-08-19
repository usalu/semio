//! ↩️ transform-node: direct sparse undo derivation limited to owned collections.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfScenesDiff, GltfNodesDiff, GltfMeshesDiff, GltfAccessorsDiff, GltfBufferViewsDiff, GltfBuffersDiff, GltfBufferBytesDiff};
use crate::artifacts::gltf::schema::mutations::transform_node::mutation::{apply, GltfTransformNodePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfTransformNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(GltfDiff { nodes: Some(GltfNodesDiff::between(&next.document.nodes, &base.document.nodes)), ..Default::default() }) }
