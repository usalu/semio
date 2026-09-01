//! 🔺️ Diff for `ChangeMaterialRoughness`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, material_at};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ChangeMaterialRoughness, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(material) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if material.roughness == payload.new_roughness {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material \"{}\" roughness factor is already {}.", payload.id, payload.new_roughness));
    }
    if !payload.new_roughness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material \"{}\" roughness factor {} is not finite.", payload.id, payload.new_roughness), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_material_roughness(base, &payload.id, payload.new_roughness))
}
//#endregion 🔖️Diff
