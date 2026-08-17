//! ⚙️ `change-material-metallic` — Error `mutation.target-missing` when material `id` is absent,
//! Warning `mutation.no-op` when `new_metallic` already equals the current value, Fatal
//! `mutation.invariant` when `new_metallic` is not finite.

use super::mutation::ChangeMaterialMetallic;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMaterialMetallic, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(material) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if material.metallic == payload.new_metallic {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material \"{}\" metallic factor is already {}.", payload.id, payload.new_metallic));
    }
    if !payload.new_metallic.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material \"{}\" metallic factor {} is not finite.", payload.id, payload.new_metallic), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_material_metallic(base, &payload.id, payload.new_metallic))
}
//#endregion 🔖️Diff
