//! ↩️ Inverse for `UnflattenNode`.

use crate::standards::v1::subsets::drawing::schema::mutations::{flatten_node, SemioDrawingMutation};
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::UnflattenNode, _base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    vec![SemioDrawingMutation::FlattenNode(flatten_node::FlattenNode { at: payload.at.clone() })]
}
//#endregion 🔖️Inverse
