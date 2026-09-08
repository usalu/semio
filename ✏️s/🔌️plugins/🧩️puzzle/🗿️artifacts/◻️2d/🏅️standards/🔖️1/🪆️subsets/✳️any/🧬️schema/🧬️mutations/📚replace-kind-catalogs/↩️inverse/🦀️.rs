//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof).
use crate::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceKindCatalogs, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::mutations::replace_kind_catalogs::replace_kind_catalogs(base.meta.kind_catalogs.clone())]
}
//#endregion 🔖️Inverse
