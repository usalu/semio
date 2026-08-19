//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof).
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::artifacts::puzzle3d::mutations::replace_kind_catalogs::mutation::replace_kind_catalogs(base.meta.kind_catalogs.clone())]
}
//#endregion 🔖️Inverse
