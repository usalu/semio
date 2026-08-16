//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — patches the whole `kindCompatibility` list.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dKindCompatibilityList};
use crate::artifacts::puzzle5d::{Puzzle5dKindCompatibility, Puzzle5dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectKindCompatibility, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "already connected").at(vec![payload.source.clone(), payload.target.clone()])]);
    }
    let mut values = base.kind_compatibility.clone();
    values.push(Puzzle5dKindCompatibility {
        source: payload.source.clone(),
        target: payload.target.clone(),
        bidirectional: payload.bidirectional,
        important: payload.important,
        specificity: payload.specificity,
    });
    protocol::MutationOutcome::new(Puzzle5dDiff { kind_compatibility: Some(Puzzle5dKindCompatibilityList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
