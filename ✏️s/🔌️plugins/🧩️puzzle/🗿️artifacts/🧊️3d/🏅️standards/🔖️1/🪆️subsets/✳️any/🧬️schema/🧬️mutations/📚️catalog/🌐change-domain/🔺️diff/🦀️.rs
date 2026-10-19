//! 🔺️ Sparse diff builder for `ChangeDomain` — patches the document `domain`.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeDomain, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if payload.new_domain == base.domain {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Domain is already \"{}\".", payload.new_domain));
    }
    protocol::MutationOutcome::new(Puzzle3dDiff { domain: Some(payload.new_domain.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
