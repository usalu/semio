//! 🔺️ Diff for `RemoveSpatialNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveSpatialNode, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::RemoveSpatialNode { id } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { spatial: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
