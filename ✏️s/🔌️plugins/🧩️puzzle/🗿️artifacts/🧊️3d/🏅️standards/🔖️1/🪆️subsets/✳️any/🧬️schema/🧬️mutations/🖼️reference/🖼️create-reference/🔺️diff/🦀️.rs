//! exists in `base`.
//! 🔺️ Sparse diff builder for `CreateReference` — a real append-only insert. No-op when the id already
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dReferencesDelta;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateReference, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.references.iter().any(|entry| entry.id == payload.reference.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "reference"), vec![payload.reference.id.clone()]);
    }
    let index = payload.index.map_or(base.references.len(), |index| index.min(base.references.len()));
    let delta = Puzzle3dReferencesDelta::insertion(index, payload.reference.clone());
    protocol::MutationOutcome::new(Puzzle3dDiff { references: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
