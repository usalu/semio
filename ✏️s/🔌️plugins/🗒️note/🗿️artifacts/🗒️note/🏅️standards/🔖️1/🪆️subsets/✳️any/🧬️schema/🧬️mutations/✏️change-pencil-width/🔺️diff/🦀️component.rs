//! 🔺️ Diff fragment yielded by `ChangePencilWidth`.
use super::mutation::ChangePencilWidth;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePencilWidth, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if let Some(width) = payload.new_width {
        if !width.is_finite() || width <= 0.0 {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pencil width must be a positive number, got {width}."), Vec::<String>::new());
        }
    }
    if payload.new_width == base.pencil_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Pencil width already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { pencil_width: Some(payload.new_width), ..Default::default() })
}
//#endregion 🔖️Diff
