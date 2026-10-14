//! 🔺️ Diff for `ChangeVortexLabel`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;
use semio_s_artifact_block_3d_diff::{Block3dVorticesDelta, Block3dVorticesPatchEntry, Block3dVortexTemplatePatch};

//#region 🔖️Diff
pub fn diff(payload: &crate::change_vortex_label::ChangeVortexLabel, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let Some(existing) = base.vortices.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex", payload.id), vec![payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block3dVortexTemplatePatch { label: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_label.clone() }), ..Default::default() };
    protocol::MutationOutcome::new(Block3dDiff { vortices: Block3dVorticesDelta { modified: vec![Block3dVorticesPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, ..Default::default() })
}
//#endregion 🔖️Diff
