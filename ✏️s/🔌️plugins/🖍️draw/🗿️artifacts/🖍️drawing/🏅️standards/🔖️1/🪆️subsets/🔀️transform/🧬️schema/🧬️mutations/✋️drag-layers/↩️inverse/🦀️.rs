//! ↩️ Inverse for `DragLayers` — `update-layer-transform` back to the BASE transform of exactly the layers the drag moves.
use semio_s_artifact_draw_drawing_geometry::{drawing_dragged_transform, drawing_placed_layers, drawing_targets_invariant};
use crate::update_layer_transform::mutation::update_layer_transform;
use semio_s_artifact_draw_drawing_model::layer_base;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::update_layer_transform::mutation::UpdateLayerTransform>>(payload: &super::mutation::DragLayers, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) || drawing_targets_invariant(&payload.targets).is_err() {
        return Ok(Vec::new());
    }
    let ids: Vec<_> = payload.targets.iter().collect();
    let (mut moved, mut restore) = (std::collections::BTreeSet::new(), Vec::new());
    for entry in drawing_placed_layers(base, &ids) {
        let source = layer_base(entry.layer);
        if entry.addressed_ancestor.is_some_and(|ancestor| moved.contains(ancestor)) {
            moved.insert(&source.id);
            continue;
        }
        if !entry.editable { continue; }
        let Some(next) = drawing_dragged_transform(&source.transform, entry.parent, [payload.dx, payload.dy]) else { continue };
        moved.insert(&source.id);
        if next != source.transform {
            restore.push(update_layer_transform(source.id.clone(), source.transform.clone()));
        }
    }
    Ok(restore)
}
//#endregion 🔖️Inverse
