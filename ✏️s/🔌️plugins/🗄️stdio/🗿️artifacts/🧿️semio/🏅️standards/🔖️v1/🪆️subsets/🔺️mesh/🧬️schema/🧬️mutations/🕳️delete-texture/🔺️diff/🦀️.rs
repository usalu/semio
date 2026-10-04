//! 🔺️ Diff for `DeleteTexture`.

use crate::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteTexture, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::standards::v1::subsets::mesh::schema::diff::texture_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Texture \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(material) = base.materials.iter().find(|material| [material.base_color_texture.as_deref(), material.metallic_roughness_texture.as_deref(), material.normal_texture.as_deref(), material.occlusion_texture.as_deref(), material.emissive_texture.as_deref()].into_iter().any(|id| id == Some(payload.id.as_str()))) {
        return protocol::MutationOutcome::error("mutation.target-referenced",format!("Texture {:?} is referenced by material {:?}.", payload.id, material.id), [payload.id.clone(), material.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::standards::v1::subsets::mesh::schema::diff::diff_remove_texture(base, &payload.id))
}
//#endregion 🔖️Diff
