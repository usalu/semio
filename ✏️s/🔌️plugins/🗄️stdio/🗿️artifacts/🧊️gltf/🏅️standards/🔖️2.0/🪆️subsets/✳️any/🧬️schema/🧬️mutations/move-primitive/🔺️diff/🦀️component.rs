//! 🔺️ move-primitive: sparse diff scoped to meshes only.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfMeshesDiff};
use crate::artifacts::gltf::schema::mutations::move_primitive::mutation::{apply, GltfMovePrimitivePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfMovePrimitivePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(GltfDiff { meshes: Some(GltfMeshesDiff::between(&base.document.meshes, &next.document.meshes)), ..Default::default() }) }
