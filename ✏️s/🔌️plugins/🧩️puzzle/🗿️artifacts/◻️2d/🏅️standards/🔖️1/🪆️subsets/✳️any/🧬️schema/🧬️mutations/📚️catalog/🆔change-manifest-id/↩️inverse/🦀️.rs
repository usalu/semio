//! ↩️ Inverse for `ChangeManifestId` — restores the BASE manifest id.
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_manifest_id::component::ChangeManifestId;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeManifestId>>(_payload: &super::ChangeManifestId, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::change_manifest_id::change_manifest_id(base.meta.manifest_id.clone())]

    })())
}
//#endregion 🔖️Inverse
