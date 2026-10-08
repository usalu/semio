//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — appends one row to `kindCompatibility`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dKindCompatibilityDelta, Puzzle5dKindCompatibilityKey};
use crate::{Puzzle5dKindCompatibility, Puzzle5dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectKindCompatibility, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "already connected").at(vec![payload.source.clone(), payload.target.clone()])]);
    }
    let row = Puzzle5dKindCompatibility { source: payload.source.clone(), target: payload.target.clone(), bidirectional: payload.bidirectional, important: payload.important, specificity: payload.specificity };
    let index = payload.index.map_or(base.kind_compatibility.len(), |index| index.min(base.kind_compatibility.len()));
    protocol::MutationOutcome::new(Puzzle5dDiff { kind_compatibility: Some(Puzzle5dKindCompatibilityDelta::insertion(index, row)), ..Default::default() })
}
//#endregion 🔖️Diff
