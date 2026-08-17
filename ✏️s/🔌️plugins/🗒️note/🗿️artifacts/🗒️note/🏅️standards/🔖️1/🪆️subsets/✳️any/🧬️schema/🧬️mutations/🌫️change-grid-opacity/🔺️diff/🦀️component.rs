//! 🔺️ Diff fragment yielded by `ChangeGridOpacity`.
use super::mutation::ChangeGridOpacity;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGridOpacity, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if let Some(opacity) = payload.new_opacity {
        if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Grid opacity must be between 0 and 1, got {opacity}."), Vec::<String>::new());
        }
    }
    if payload.new_opacity == base.grid_opacity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Grid opacity already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { grid_opacity: Some(payload.new_opacity), ..Default::default() })
}
//#endregion 🔖️Diff
