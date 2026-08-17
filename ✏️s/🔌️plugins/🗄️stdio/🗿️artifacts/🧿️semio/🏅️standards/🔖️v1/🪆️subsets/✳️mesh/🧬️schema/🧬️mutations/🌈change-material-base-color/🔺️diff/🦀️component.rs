//! 🌈 `change-material-base-color` — Error `mutation.target-missing` when material `id` is
//! absent, Warning `mutation.no-op` when `new_base_color` already equals the current value, Fatal
//! `mutation.invariant` when any `new_base_color` channel is not finite.

use super::mutation::ChangeMaterialBaseColor;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMaterialBaseColor, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(material) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if material.base_color == payload.new_base_color {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material \"{}\" base color is unchanged.", payload.id));
    }
    let c = payload.new_base_color;
    if !c.r.is_finite() || !c.g.is_finite() || !c.b.is_finite() || !c.a.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material \"{}\" base color has a non-finite channel.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_material_base_color(base, &payload.id, payload.new_base_color))
}
//#endregion 🔖️Diff
