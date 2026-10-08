//! 🔺️ Diff for `SetSpatialNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetSpatialNode, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    if !base.spatial.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Spatial node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::SetSpatialNode { id, kind, name, parent_id, placement } = payload;
    protocol::MutationOutcome::new(SemioModelDiff {
        spatial: Some(NamedTripleDiff { modified: vec![NamedModified { key: id.clone(), diff: SpatialNodeDiff { kind: *kind, name: name.clone(), parent_id: parent_id.clone(), placement: *placement } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
