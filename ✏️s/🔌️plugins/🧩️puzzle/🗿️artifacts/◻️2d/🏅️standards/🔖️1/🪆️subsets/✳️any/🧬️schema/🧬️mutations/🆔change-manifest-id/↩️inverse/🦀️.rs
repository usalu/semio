//! ↩️ Inverse for `ChangeManifestId` — restores the BASE manifest id.
use crate::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeManifestId, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::mutations::change_manifest_id::change_manifest_id(base.meta.manifest_id.clone())]
}
//#endregion 🔖️Inverse
