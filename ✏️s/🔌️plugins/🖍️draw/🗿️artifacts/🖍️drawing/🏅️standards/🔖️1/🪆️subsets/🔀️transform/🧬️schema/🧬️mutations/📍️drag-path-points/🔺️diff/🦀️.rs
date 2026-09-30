//! 🔺️ Sparse diff builder for `DragPathPoints` — every addressed point of every surviving path moves by the offset mapped
//! into the path's own axes, read off the BASE geometry; an anchor carries its attached tangents.
use crate::mutations::drawing_placed_layers;
use crate::schema::geometry::editing::{translate_world_path_points, PathPointRef};
use crate::schema::layer_base;
use crate::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DragPathPoints, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
    let layers = super::mutation::drag_path_points_layers(&payload.targets);
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", layers);
    }
    if payload.targets.is_empty() || payload.targets.iter().enumerate().any(|(index, target)| payload.targets[..index].contains(target)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a path point drag addresses at least one point, every point once", layers);
    }
    let ids: Vec<&str> = layers.iter().map(String::as_str).collect();
    let placed = drawing_placed_layers(base, &ids);
    let (mut missing, mut locked, mut unmatched, mut patched) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut applies = false;
    for id in &layers {
        let points: Vec<PathPointRef> = payload.targets.iter().filter(|target| &target.layer_id == id).map(|target| PathPointRef { index: target.index, point: target.point }).collect();
        match placed.iter().find(|entry| &layer_base(entry.layer).id == id) {
            None => missing.push(id.clone()),
            Some(entry) if !entry.editable => locked.push(id.clone()),
            Some(entry) => match entry.layer {
                DrawingLayerNode::Path(path) => {
                    let matrix = crate::schema::geometry::multiply(entry.parent, crate::schema::drawing_transform_to_matrix(&path.base.transform));
                    match translate_world_path_points(&path.segments, &points, matrix, [payload.dx, payload.dy]) {
                        Ok(segments) => {
                            applies = true;
                            if segments != path.segments {
                                patched.push((id.clone(), segments));
                            }
                        }
                        Err(_) => unmatched.push(id.clone()),
                    }
                }
                _ => unmatched.push(id.clone()),
            },
        }
    }
    if !applies {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} addressed path(s) is a visible, unlocked path owning the dragged points", layers.len()), layers);
    }
    let partial: Vec<protocol::MutationMessage> = [(missing, "not in this drawing"), (locked, "locked or hidden"), (unmatched, "no longer a path owning these points")]
        .into_iter()
        .filter(|(skipped, _)| !skipped.is_empty())
        .map(|(skipped, reason)| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} path(s) skipped ({reason}): {}", skipped.len(), layers.len(), skipped.join(", "))).at(skipped))
        .collect();
    if patched.is_empty() {
        return protocol::MutationOutcome::new(crate::diff::DrawingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "no path point moves").at(layers)]));
    }
    protocol::MutationOutcome::new(crate::diff::diff_set_path_geometries(patched)).absorb_messages(partial)
}
//#endregion 🔖️Diff
