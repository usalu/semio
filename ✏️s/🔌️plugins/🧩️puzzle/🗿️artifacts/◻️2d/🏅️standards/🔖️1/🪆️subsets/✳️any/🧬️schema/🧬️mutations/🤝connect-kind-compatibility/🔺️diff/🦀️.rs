//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — appends one row to `meta.kindCompatibility`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dKindCompatibilityDelta, Puzzle2dMetaPatch};
use crate::{Puzzle2dKindCompatibility, Puzzle2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectKindCompatibility, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if base.meta.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "already connected").at(vec![payload.source.to_string_owner(), payload.target.to_string_owner()])]);
    }
    let row = Puzzle2dKindCompatibility { source: payload.source.clone(), target: payload.target.clone(), bidirectional: payload.bidirectional, important: payload.important, specificity: payload.specificity };
    let index = payload.index.map_or(base.meta.kind_compatibility.len(), |index| index.min(base.meta.kind_compatibility.len()));
    protocol::MutationOutcome::new(Puzzle2dDiff { meta: Some(Puzzle2dMetaPatch { kind_compatibility: Some(Puzzle2dKindCompatibilityDelta::insertion(index, row)), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
