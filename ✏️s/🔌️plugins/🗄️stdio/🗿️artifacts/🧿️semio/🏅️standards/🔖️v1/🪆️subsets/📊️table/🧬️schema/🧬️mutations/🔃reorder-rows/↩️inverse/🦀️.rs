//! ↩️ Inverse for `ReorderRows`.

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReorderRows, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let len = base.rows.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![SemioTableMutation::ReorderRows(super::ReorderRows { from: landed_at, to: payload.from })]
}
//#endregion 🔖️Inverse
