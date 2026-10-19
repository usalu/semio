//! 🔺️ Sparse diff builder for `ScaleTargetVolume` — patches the one addressed target-volume in place.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dTargetVolumePatch;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dTargetVolumesDelta;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleTargetVolume, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target-volume", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle3dTargetVolumePatch {
        scale: (payload.new_scale != item.scale).then_some(payload.new_scale),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        target_volumes: Some(Puzzle3dTargetVolumesDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
