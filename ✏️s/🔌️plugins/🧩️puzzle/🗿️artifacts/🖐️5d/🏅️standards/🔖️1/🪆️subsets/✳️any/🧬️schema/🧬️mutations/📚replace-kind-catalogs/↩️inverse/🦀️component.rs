//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof).
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::artifacts::puzzle5d::mutations::replace_kind_catalogs::mutation::replace_kind_catalogs(base.kind_catalogs.clone())]
}
//#endregion 🔖️Inverse
