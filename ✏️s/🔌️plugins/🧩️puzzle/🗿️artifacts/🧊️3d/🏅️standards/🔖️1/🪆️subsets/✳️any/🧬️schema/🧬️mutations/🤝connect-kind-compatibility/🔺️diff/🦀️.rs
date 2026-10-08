//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — appends one row to `meta.kindCompatibility`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dKindCompatibilityDelta, Puzzle3dKindCompatibilityKey, Puzzle3dMetaPatch};
use crate::{Puzzle3dKindCompatibility, Puzzle3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectKindCompatibility, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.meta.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "already connected").at(vec![payload.source.clone(), payload.target.clone()])]);
    }
    let row = Puzzle3dKindCompatibility { source: payload.source.clone(), target: payload.target.clone(), bidirectional: payload.bidirectional, important: payload.important, specificity: payload.specificity };
    let index = payload.index.map_or(base.meta.kind_compatibility.len(), |index| index.min(base.meta.kind_compatibility.len()));
    protocol::MutationOutcome::new(Puzzle3dDiff { meta: Some(Puzzle3dMetaPatch { kind_compatibility: Some(Puzzle3dKindCompatibilityDelta::insertion(index, row)), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
