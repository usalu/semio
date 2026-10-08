//! 🔺️ Sparse diff builder for `DisconnectKindCompatibility` — removes one row from `kindCompatibility`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dKindCompatibilityDelta, Puzzle5dKindCompatibilityKey};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectKindCompatibility, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !base.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} not found", "kind-compatibility"), vec![payload.source.clone(), payload.target.clone()]);
    }
    let key = Puzzle5dKindCompatibilityKey { source: payload.source.clone(), target: payload.target.clone() };
    protocol::MutationOutcome::new(Puzzle5dDiff { kind_compatibility: Some(Puzzle5dKindCompatibilityDelta::removing(vec![key])), ..Default::default() })
}
//#endregion 🔖️Diff
