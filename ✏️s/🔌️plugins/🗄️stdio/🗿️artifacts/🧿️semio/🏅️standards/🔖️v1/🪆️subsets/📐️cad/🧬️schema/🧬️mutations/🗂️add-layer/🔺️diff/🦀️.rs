//! 🔺️ Diff for `AddLayer`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    let super::AddLayer { layer } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![layer.clone()] }), blocks: None, entities: None })
}
//#endregion 🔖️Diff
