//! 🔺️ Diff for `RemoveBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveBlock, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::RemoveBlock { name } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: Some(NamedTripleDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }), entities: None })
}
//#endregion 🔖️Diff
