//! 🔺️ Diff fragment yielded by `ChangeGridVisible`.
use super::ChangeGridVisible;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGridVisible, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if payload.new_visible == base.grid_visible {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Grid visibility already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { grid_visible: Some(crate::schema::diff::NoteAssigned::new(payload.new_visible)), ..Default::default() })
}
//#endregion 🔖️Diff
