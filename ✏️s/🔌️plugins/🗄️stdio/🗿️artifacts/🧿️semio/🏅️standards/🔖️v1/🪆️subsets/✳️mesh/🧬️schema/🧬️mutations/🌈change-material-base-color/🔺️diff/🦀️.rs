//! 🔺️ Diff for `ChangeMaterialBaseColor`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, material_at};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ChangeMaterialBaseColor, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
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
