//! ↩️ Inverse for `SetQuery` — a `set-query` back to the query BASE held.
use crate::standards::v1::subsets::any::schema::mutations::{set_query, TrinityGraphMutation};
use crate::JackSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::SetQuery, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    vec![set_query(base.query.clone())]
}
//#endregion 🔖️Inverse
