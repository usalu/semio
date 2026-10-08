//! 🔺️ Diff for `AddEntity`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddEntity, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::AddEntity { entity } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: None, entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![entity.clone()] }) })
}
//#endregion 🔖️Diff
