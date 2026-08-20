//! 📐 `replace-primitive-geometry` — Error `mutation.target-missing` when the
//! (`mesh_id`,`primitive_id`) pair is absent, Warning `mutation.no-op` when every new geometry
//! buffer already equals the current one (all five buffer types derive `PartialEq`, so the
//! comparison is a cheap field-wise `==` — the same style `schema::diff::between_primitive`
//! already uses for its own `between()`).

use super::mutation::ReplacePrimitiveGeometry;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ReplacePrimitiveGeometry, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(primitive) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at(base, &payload.mesh_id, &payload.primitive_id).await else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Primitive \"{}\" does not exist in mesh \"{}\".", payload.primitive_id, payload.mesh_id), [payload.primitive_id.clone()]).await;
    };
    if primitive.positions == payload.positions && primitive.normals == payload.normals && primitive.uvs == payload.uvs && primitive.colors == payload.colors && primitive.indices == payload.indices {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Primitive \"{}\" geometry is unchanged.", payload.primitive_id)).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_replace_primitive_geometry(
        base,
        &payload.mesh_id,
        &payload.primitive_id,
        payload.positions.clone(),
        payload.normals.clone(),
        payload.uvs.clone(),
        payload.colors.clone(),
        payload.indices.clone(),
    ))
}
//#endregion 🔖️Diff
