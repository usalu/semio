//! 🔺️ Diff for `InsertSpatialNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertSpatialNode, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::InsertSpatialNode { node, at } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { spatial: Some(NamedTripleDiff { added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.spatial.len(), |at| at.min(base.spatial.len())), item: node.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
