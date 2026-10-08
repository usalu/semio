//! 🔺️ Diff for `CreateMaterial`.

use crate::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateMaterial, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.material.id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    for id in [payload.material.base_color_texture.as_deref(), payload.material.metallic_roughness_texture.as_deref(), payload.material.normal_texture.as_deref(), payload.material.occlusion_texture.as_deref(), payload.material.emissive_texture.as_deref()].into_iter().flatten() {
        if !base.textures.iter().any(|texture| texture.id == id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Material texture {id:?} does not exist."), [payload.material.id.clone(), id.to_owned()]);
        }
    }
    protocol::MutationOutcome::new(crate::standards::v1::subsets::mesh::schema::diff::diff_add_material(base, payload.material.clone(), payload.at))
}
//#endregion 🔖️Diff
