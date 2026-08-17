//! 🔺️ Diff fragment yielded by `ChangeSnapGridSpacing`.
use super::mutation::ChangeSnapGridSpacing;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnapGridSpacing, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if let Some(spacing) = payload.new_spacing {
        if !spacing.is_finite() || spacing <= 0.0 {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Snap grid spacing must be a positive number, got {spacing}."), Vec::<String>::new());
        }
    }
    if payload.new_spacing == base.snap_grid_spacing {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Snap grid spacing already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { snap_grid_spacing: Some(payload.new_spacing), ..Default::default() })
}
//#endregion 🔖️Diff
