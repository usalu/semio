//! ↩️ Inverse for `ScaleLayers` — `update-layer-transform` back to the BASE transform of exactly the layers the scaling reshapes.
use crate::mutations::{drawing_moved_transform, drawing_placed_layers, drawing_scaling_matrix, drawing_targets_invariant, update_layer_transform, DrawingMutation};
use crate::schema::layer_base;
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ScaleLayers, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    if ![payload.pivot_x, payload.pivot_y, payload.scale_x, payload.scale_y].iter().all(|value| value.is_finite()) || payload.scale_x == 0.0 || payload.scale_y == 0.0 || drawing_targets_invariant(&payload.targets).is_err() {
        return Ok(Vec::new());
    }
    let motion = drawing_scaling_matrix(payload.pivot_x, payload.pivot_y, payload.scale_x, payload.scale_y);
    let identity = payload.scale_x == 1.0 && payload.scale_y == 1.0;
    let ids: Vec<_> = payload.targets.iter().collect();
    let (mut moved, mut restore) = (std::collections::BTreeSet::new(), Vec::new());
    for entry in drawing_placed_layers(base, &ids) {
        let source = layer_base(entry.layer);
        if !entry.editable || entry.addressed_ancestor.is_some_and(|ancestor| moved.contains(ancestor)) {
            continue;
        }
        let Some(next) = (if identity { Some(source.transform.clone()) } else { drawing_moved_transform(&source.transform, entry.parent, motion) }) else { continue };
        moved.insert(&source.id);
        if next != source.transform {
            restore.push(update_layer_transform(source.id.clone(), source.transform.clone()));
        }
    }
    Ok(restore)
}
//#endregion 🔖️Inverse
