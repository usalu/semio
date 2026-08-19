//! ↩️ change-mesh-morph-weights: sparse diff scoped to meshes only.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfMeshesDiff};
use crate::artifacts::gltf::schema::mutations::change_mesh_morph_weights::mutation::{apply, GltfChangeMeshMorphWeightsPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfChangeMeshMorphWeightsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(GltfDiff { meshes: Some(GltfMeshesDiff::between(&next.document.meshes, &base.document.meshes)), ..Default::default() }) }
