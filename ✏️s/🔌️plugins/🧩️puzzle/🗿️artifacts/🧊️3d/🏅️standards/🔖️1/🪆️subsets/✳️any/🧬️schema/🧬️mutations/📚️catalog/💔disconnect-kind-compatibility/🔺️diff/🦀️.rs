//! 🔺️ Sparse diff builder for `DisconnectKindCompatibility` — removes one row from `meta.kindCompatibility`.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dKindCompatibilityDelta;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dKindCompatibilityKey;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dMetaPatch;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(at) = base.meta.kind_compatibility.iter().position(|row| row.source == payload.source && row.target == payload.target) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} not found", "kind-compatibility"), vec![payload.source.clone(), payload.target.clone()]);
    };
    let key = Puzzle3dKindCompatibilityKey { source: payload.source.clone(), target: payload.target.clone() };
    protocol::MutationOutcome::new(Puzzle3dDiff { meta: Some(Puzzle3dMetaPatch { kind_compatibility: Some(Puzzle3dKindCompatibilityDelta::removal_by_id(key, at)), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
