//! 🔺️ Diff fragment yielded by `ChangeGridSubdivisions`.
use super::mutation::ChangeGridSubdivisions;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeGridSubdivisions, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if let Some(subdivisions) = payload.new_subdivisions {
        if !subdivisions.is_finite() || subdivisions < 1.0 {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Grid subdivisions must be at least 1, got {subdivisions}."), Vec::<String>::new());
        }
    }
    if payload.new_subdivisions == base.grid_subdivisions {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Grid subdivisions already has this value.");
    }
    protocol::MutationOutcome::new(NoteDiff { grid_subdivisions: Some(payload.new_subdivisions), ..Default::default() })
}
//#endregion 🔖️Diff
