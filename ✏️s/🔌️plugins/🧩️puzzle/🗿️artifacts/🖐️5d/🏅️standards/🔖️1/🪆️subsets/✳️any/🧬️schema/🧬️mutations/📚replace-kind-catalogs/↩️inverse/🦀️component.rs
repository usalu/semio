//! ↩️ Inverse for `ReplaceKindCatalogs` — restores the BASE catalogs (or lack thereof). Reassembles
//! `base`'s composed `kind_catalogs` handle + `kind_catalogs_extra` overflow back into the payload's
//! original `Option<Puzzle5dKindCatalogs>` shape via `kind_catalogs_of` (working-scene cache read).
use crate::artifacts::puzzle5d::kind_catalogs_of;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let restored = kind_catalogs_of(&base.kind_catalogs, &base.kind_catalogs_extra);
    vec![crate::artifacts::puzzle5d::mutations::replace_kind_catalogs::mutation::replace_kind_catalogs(restored)]
}
//#endregion 🔖️Inverse
