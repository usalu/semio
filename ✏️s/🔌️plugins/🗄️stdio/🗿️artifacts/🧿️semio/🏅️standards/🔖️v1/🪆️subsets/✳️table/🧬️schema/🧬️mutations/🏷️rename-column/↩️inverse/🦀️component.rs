//! ↩️ `rename-column` — undo swaps `name`/`new_name`, looking up the OLD name's presence in
//! `base` (`📓️taxonomy.md`'s addressing rule #2: "`rename`'s inverse looks up the OLD name from
//! `base`"). Column absent from `base` ⇒ `Vec::new()`.

use super::mutation::RenameColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &RenameColumn, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    if !base.columns.iter().any(|c| c.name == payload.name) {
        return Vec::new();
    }
    vec![SemioTableMutation::RenameColumn(RenameColumn { name: payload.new_name.clone(), new_name: payload.name.clone() })]
}
//#endregion 🔖️Inverse
