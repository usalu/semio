//! 🔺️ Diff for `AddBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddBlock, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::AddBlock { block } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![block.clone()] }), entities: None })
}
//#endregion 🔖️Diff
