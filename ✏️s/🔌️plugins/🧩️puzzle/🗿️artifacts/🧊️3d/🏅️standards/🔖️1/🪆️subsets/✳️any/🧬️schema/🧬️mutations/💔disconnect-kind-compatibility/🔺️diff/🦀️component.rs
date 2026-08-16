//! 🔺️ Sparse diff builder for `DisconnectKindCompatibility` — patches the document `meta.kindCompatibility`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !base.meta.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} not found", "kind-compatibility"), vec![payload.source.clone(), payload.target.clone()]);
    }
    let mut meta = base.meta.clone();
    meta.kind_compatibility.retain(|row| !(row.source == payload.source && row.target == payload.target));
    protocol::MutationOutcome::new(Puzzle3dDiff { meta: Some(meta), ..Default::default() })
}
//#endregion 🔖️Diff
