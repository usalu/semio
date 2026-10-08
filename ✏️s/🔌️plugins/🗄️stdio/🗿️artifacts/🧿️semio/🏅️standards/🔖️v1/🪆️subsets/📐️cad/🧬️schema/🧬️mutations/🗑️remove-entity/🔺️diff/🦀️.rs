//! 🔺️ Diff for `RemoveEntity`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveEntity, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::RemoveEntity { handle } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: None, entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }) })
}
//#endregion 🔖️Diff
