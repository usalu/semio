//! 🔺️ Sparse diff builder for `DisconnectKindCompatibility` — patches the whole `kindCompatibility` list.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dKindCompatibilityList};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !base.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} not found", "kind-compatibility"), vec![payload.source.clone(), payload.target.clone()]);
    }
    let values: Vec<_> = base.kind_compatibility.iter().cloned().filter(|row| !(row.source == payload.source && row.target == payload.target)).collect();
    protocol::MutationOutcome::new(Puzzle5dDiff { kind_compatibility: Some(Puzzle5dKindCompatibilityList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
