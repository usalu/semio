//! 🔺️ Sparse diff builder for `DragLayers` — every surviving layer's origin moves by the offset mapped into its parent's
//! axes, read off the BASE transform, so the leaf replays on any base.
use crate::diff::{diff_set_layer_transforms, DrawingDiff};
use crate::mutations::{drawing_dragged_transform, drawing_placed_layers, drawing_selection_partial, drawing_targets_invariant};
use crate::schema::layer_base;
use crate::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DragLayers, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let targets: Vec<String> = payload.targets.iter().map(|id| id.to_string_owner()).collect();
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", targets);
    }
    if let Err(reason) = drawing_targets_invariant(&payload.targets) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, targets);
    }
    let ids: Vec<_> = payload.targets.iter().collect();
    let placed = drawing_placed_layers(base, &ids);
    let (mut moved, mut locked, mut singular, mut rows, mut applies) = (std::collections::BTreeSet::new(), Vec::new(), Vec::new(), Vec::new(), false);
    for entry in &placed {
        let source = layer_base(entry.layer);
        if entry.addressed_ancestor.is_some_and(|ancestor| moved.contains(ancestor)) {
            applies = true;
            continue;
        }
        if !entry.editable {
            locked.push(source.id.to_string_owner());
            continue;
        }
        match drawing_dragged_transform(&source.transform, entry.parent, [payload.dx, payload.dy]) {
            Some(next) => {
                applies = true;
                moved.insert(&source.id);
                if next != source.transform {
                    rows.push((source.id.to_string_owner(), next));
                }
            }
            None => singular.push(source.id.to_string_owner()),
        }
    }
    if !applies {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a visible, unlocked layer this transform can place", targets.len()), targets);
    }
    let partial = drawing_selection_partial(&payload.targets, &placed, locked, singular);
    if rows.is_empty() {
        return protocol::MutationOutcome::new(DrawingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no layer changes its transform").at(targets)]));
    }
    protocol::MutationOutcome::new(diff_set_layer_transforms(rows)).absorb_messages(partial)
}
//#endregion 🔖️Diff
