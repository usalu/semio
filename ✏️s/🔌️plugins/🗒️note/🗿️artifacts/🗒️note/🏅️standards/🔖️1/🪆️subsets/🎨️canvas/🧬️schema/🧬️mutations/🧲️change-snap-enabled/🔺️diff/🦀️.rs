//! 🔺️ Diff fragment yielded by `ChangeSnapEnabled`.
use super::ChangeSnapEnabled;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnapEnabled, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if payload.new_enabled == base.snap_enabled {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Snap enabled already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { snap_enabled: Some(payload.new_enabled), ..Default::default() })
}
//#endregion 🔖️Diff
