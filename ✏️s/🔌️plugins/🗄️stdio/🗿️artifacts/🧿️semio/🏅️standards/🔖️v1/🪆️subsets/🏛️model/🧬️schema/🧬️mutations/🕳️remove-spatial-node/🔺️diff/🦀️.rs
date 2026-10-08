//! 🔺️ Diff for `RemoveSpatialNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveSpatialNode, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    if !base.spatial.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Spatial node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::RemoveSpatialNode { id } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { spatial: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
