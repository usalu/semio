//! 🦀 bind-primitive-indices: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.bind-primitive-indices.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfBindPrimitiveIndicesPayload { pub mesh: usize, pub primitive: usize, pub accessor: usize }
pub async fn validate(payload: &GltfBindPrimitiveIndicesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; checked_index(payload.accessor, base.document.accessors.len(), "document/accessors")?; if base.document.accessors[payload.accessor].kind != crate::artifacts::gltf::engine::GltfAccessorType::Scalar || base.document.accessors[payload.accessor].component_type == crate::artifacts::gltf::engine::GltfComponentType::F32 { return Err(reject("gltf.mutation.invalid-index-accessor", "document/accessors", "indices require a scalar integer accessor")); } Ok(()) }
pub async fn apply(payload: &GltfBindPrimitiveIndicesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].indices = Some(payload.accessor); Ok(next) }
