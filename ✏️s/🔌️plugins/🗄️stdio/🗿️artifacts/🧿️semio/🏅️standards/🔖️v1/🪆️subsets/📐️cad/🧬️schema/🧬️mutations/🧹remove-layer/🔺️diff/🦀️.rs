//! 🔺️ Diff for `RemoveLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::RemoveLayer { name } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: Some(NamedTripleDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }), blocks: None, entities: None })
}
//#endregion 🔖️Diff
