//! ↩️ Inverse for `ChangeManifestId` — restores the BASE manifest id.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangeManifestId, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::artifacts::puzzle2d::mutations::change_manifest_id::mutation::change_manifest_id(base.meta.manifest_id.clone())]
}
//#endregion 🔖️Inverse
