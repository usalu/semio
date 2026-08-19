//! ↩️ reorder-primitives: sparse diff scoped to meshes only.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfMeshesDiff};
use crate::artifacts::gltf::schema::mutations::reorder_primitives::mutation::{apply, GltfReorderPrimitivesPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(GltfDiff { meshes: Some(GltfMeshesDiff::between(&next.document.meshes, &base.document.meshes)), ..Default::default() }) }
