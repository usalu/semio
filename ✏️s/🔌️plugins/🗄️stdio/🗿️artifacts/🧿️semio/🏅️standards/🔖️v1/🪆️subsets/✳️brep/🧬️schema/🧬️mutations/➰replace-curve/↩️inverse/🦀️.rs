//! ↩️ Inverse for `ReplaceCurve`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, SemioBrepSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReplaceCurve, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    match base.edges.iter().find(|e| e.id == payload.edge_id) {
        Some(edge) => vec![SemioBrepMutation::ReplaceCurve(super::ReplaceCurve { edge_id: payload.edge_id.clone(), new_curve: edge.curve.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
