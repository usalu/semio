//! ↩️ `drag-nodes` — undo is the same node set dragged by the negated offset (`taxonomy.md`'s
//! `drag` row: "inverse = drag (negated offset)"), independent of `base` (a node missing from
//! `base` is already a no-op in `diff`, and stays a no-op here too).

use super::mutation::DragNodes;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DragNodes, _base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    vec![SemioDrawingMutation::DragNodes(DragNodes { ats: payload.ats.clone(), offset: SemioPoint2 { x: -payload.offset.x, y: -payload.offset.y } })]
}
//#endregion 🔖️Inverse
