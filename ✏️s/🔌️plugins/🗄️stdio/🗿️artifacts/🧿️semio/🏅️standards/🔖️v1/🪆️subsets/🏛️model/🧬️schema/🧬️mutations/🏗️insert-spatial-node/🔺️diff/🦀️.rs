//! 🔺️ Diff for `InsertSpatialNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertSpatialNode, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::InsertSpatialNode { node } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { spatial: Some(NamedTripleDiff { added: vec![node.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
