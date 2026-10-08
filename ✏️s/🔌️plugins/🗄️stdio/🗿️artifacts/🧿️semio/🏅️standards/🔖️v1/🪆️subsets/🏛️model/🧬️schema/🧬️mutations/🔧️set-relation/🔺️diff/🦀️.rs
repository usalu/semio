//! 🔺️ Diff for `SetRelation`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetRelation, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::SetRelation { id, kind, from, to } = payload;
    protocol::MutationOutcome::new({
        SemioModelDiff { relations: Some(NamedTripleDiff { modified: vec![NamedModified { key: id.clone(), diff: ModelRelationDiff { kind: kind.clone(), from: from.clone(), to: to.clone() } }], ..Default::default() }), ..Default::default() }
    })
}
//#endregion 🔖️Diff
