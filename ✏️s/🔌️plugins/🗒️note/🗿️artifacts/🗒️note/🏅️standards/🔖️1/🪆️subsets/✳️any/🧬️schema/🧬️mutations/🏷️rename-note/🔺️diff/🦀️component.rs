//! 🔺️ Diff fragment yielded by `RenameNote`. Warning `no-op` when the title is unchanged.
use super::mutation::RenameNote;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameNote, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if base.title == payload.new_title {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Note title is already {:?}.", payload.new_title));
    }
    protocol::MutationOutcome::new(NoteDiff { title: Some(payload.new_title.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
