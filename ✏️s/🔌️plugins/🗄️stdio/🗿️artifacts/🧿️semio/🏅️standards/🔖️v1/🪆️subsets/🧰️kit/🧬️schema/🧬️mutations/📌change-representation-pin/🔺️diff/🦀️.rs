//! 🔺️ Diff for `ChangeRepresentationPin`.

use semio_s_artifact_stdio_semio_base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::schema::diff::{SemioKitDiff, SemioKitLinkDiff};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ChangeRepresentationPin, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    let Some(existing) = base.representations.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No representation link exists at index #{}.", payload.index), [payload.index.to_string()]);
    };
    if existing.pin == payload.pin {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Representation link #{} is already pinned to that value.", payload.index));
    }
    protocol::MutationOutcome::new(SemioKitDiff { representations: Some(IndexedTripleDiff { modified: vec![IndexModified { index: payload.index, diff: SemioKitLinkDiff { pin: Some(payload.pin.clone()) } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
