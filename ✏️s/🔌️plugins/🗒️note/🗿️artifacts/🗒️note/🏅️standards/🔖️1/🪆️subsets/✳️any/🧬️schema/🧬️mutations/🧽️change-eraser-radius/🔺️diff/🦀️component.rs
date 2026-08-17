//! 🔺️ Diff fragment yielded by `ChangeEraserRadius`.
use super::mutation::ChangeEraserRadius;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEraserRadius, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if let Some(radius) = payload.new_radius {
        if !radius.is_finite() || radius <= 0.0 {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Eraser radius must be a positive number, got {radius}."), Vec::<String>::new());
        }
    }
    if payload.new_radius == base.eraser_radius {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Eraser radius already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { eraser_radius: Some(payload.new_radius), ..Default::default() })
}
//#endregion 🔖️Diff
