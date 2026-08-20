//! 🎨 `create-material` — Fatal `mutation.duplicate-id` when material `id` already exists.

use super::mutation::CreateMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &CreateMaterial, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.material.id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_material(base, payload.material.clone()))
}
//#endregion 🔖️Diff
