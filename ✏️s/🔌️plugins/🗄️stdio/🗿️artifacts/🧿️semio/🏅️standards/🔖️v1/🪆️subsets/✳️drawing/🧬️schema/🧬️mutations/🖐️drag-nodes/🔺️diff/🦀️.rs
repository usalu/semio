//! 🔺️ Diff for `DragNodes`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{NodePath, SemioDrawingDiff, diff_move_node, node_origin};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use protocol::MutationDiff;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DragNodes, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    if !payload.offset.x.is_finite() || !payload.offset.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Drag offset has a non-finite component.".to_string(), payload.ats.iter().map(|a| a.layer.to_string()));
    }
    if payload.offset.x == 0.0 && payload.offset.y == 0.0 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Drag offset is zero; nothing moved.".to_string());
    }
    let mut acc = SemioDrawingDiff::default();
    let mut any_resolved = false;
    for at in &payload.ats {
        if let Some(origin) = node_origin(base, at) {
            any_resolved = true;
            let new_origin = SemioPoint2 { x: origin.x + payload.offset.x, y: origin.y + payload.offset.y };
            acc.absorb(diff_move_node(base, at, new_origin));
        }
    }
    if !payload.ats.is_empty() && !any_resolved {
        return protocol::MutationOutcome::error("mutation.target-missing", "None of the addressed nodes exist.".to_string(), payload.ats.iter().map(|a| a.layer.to_string()));
    }
    protocol::MutationOutcome::new(acc)
}
//#endregion 🔖️Diff
