//! 🔺️ Diff for `InsertRelation`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertRelation, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::InsertRelation { relation } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { relations: Some(NamedTripleDiff { added: vec![relation.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
