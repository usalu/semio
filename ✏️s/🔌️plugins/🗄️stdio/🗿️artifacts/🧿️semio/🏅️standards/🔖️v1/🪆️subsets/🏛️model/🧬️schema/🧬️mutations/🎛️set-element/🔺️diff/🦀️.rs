//! 🔺️ Diff for `SetElement`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetElement, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    if !base.elements.iter().any(|e| e.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::SetElement { id, class, placement, geometry, spatial_id, psets } = payload;
    protocol::MutationOutcome::new(SemioModelDiff {
        elements: Some(NamedTripleDiff {
            modified: vec![NamedModified { key: id.clone(), diff: SemioModelElementDiff { class: class.clone(), placement: *placement, geometry: geometry.clone(), spatial_id: spatial_id.clone(), psets: psets.clone() } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
