//! ↩️ change-mesh-extra-data: sparse diff scoped to meshes only.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfMeshesDiff};
use crate::artifacts::gltf::schema::mutations::change_mesh_extra_data::mutation::{apply, GltfChangeMeshExtraDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfChangeMeshExtraDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(GltfDiff { meshes: Some(GltfMeshesDiff::between(&next.document.meshes, &base.document.meshes)), ..Default::default() }) }
