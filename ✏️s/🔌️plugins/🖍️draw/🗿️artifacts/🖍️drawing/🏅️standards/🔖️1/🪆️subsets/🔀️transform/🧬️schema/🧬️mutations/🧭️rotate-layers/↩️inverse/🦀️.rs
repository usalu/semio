//! ↩️ Inverse for `RotateLayers` — `update-layer-transform` back to the BASE transform of exactly the layers the rotation turns.
use crate::mutations::{drawing_moved_transform, drawing_placed_layers, drawing_rotation_matrix, drawing_targets_invariant, update_layer_transform, DrawingMutation};
use crate::schema::layer_base;
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RotateLayers, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) || drawing_targets_invariant(&payload.targets).is_err() {
        return Ok(Vec::new());
    }
    let motion = drawing_rotation_matrix(payload.pivot_x, payload.pivot_y, payload.angle);
    let ids: Vec<_> = payload.targets.iter().collect();
    let (mut moved, mut restore) = (std::collections::BTreeSet::new(), Vec::new());
    for entry in drawing_placed_layers(base, &ids) {
        let source = layer_base(entry.layer);
        if !entry.editable || entry.addressed_ancestor.is_some_and(|ancestor| moved.contains(ancestor)) {
            continue;
        }
        let Some(next) = (if payload.angle == 0.0 { Some(source.transform.clone()) } else { drawing_moved_transform(&source.transform, entry.parent, motion) }) else { continue };
        moved.insert(&source.id);
        if next != source.transform {
            restore.push(update_layer_transform(source.id.clone(), source.transform.clone()));
        }
    }
    Ok(restore)
}
//#endregion 🔖️Inverse
