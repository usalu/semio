//! 🔺️ Diff fragment yielded by `ChangeGridVisible`.
use super::mutation::ChangeGridVisible;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeGridVisible, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if payload.new_visible == base.grid_visible {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Grid visibility already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { grid_visible: Some(payload.new_visible), ..Default::default() })
}
//#endregion 🔖️Diff
