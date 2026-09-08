//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof).
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::mutation::replace_kind_catalogs(base.meta.kind_catalogs.clone())]
}
//#endregion 🔖️Inverse
