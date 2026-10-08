//! 🔺️ Sparse diff builder for `DisconnectKindCompatibility` — removes one row from `meta.kindCompatibility`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dKindCompatibilityDelta, Puzzle2dKindCompatibilityKey, Puzzle2dMetaPatch};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectKindCompatibility, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(at) = base.meta.kind_compatibility.iter().position(|row| row.source == payload.source && row.target == payload.target) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} not found", "kind-compatibility"), vec![payload.source.to_string_owner(), payload.target.to_string_owner()]);
    };
    let key = Puzzle2dKindCompatibilityKey { source: payload.source.clone(), target: payload.target.clone() };
    protocol::MutationOutcome::new(Puzzle2dDiff { meta: Some(Puzzle2dMetaPatch { kind_compatibility: Some(Puzzle2dKindCompatibilityDelta::removal_by_id(key, at)), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
