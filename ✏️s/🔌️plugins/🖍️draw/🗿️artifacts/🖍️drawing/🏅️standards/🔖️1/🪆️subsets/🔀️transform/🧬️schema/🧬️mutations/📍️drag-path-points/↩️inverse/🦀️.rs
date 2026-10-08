//! ↩️ Inverse for `DragPathPoints` — `update-path-geometry` back to the BASE segments of exactly the paths the drag reshapes.
use crate::mutations::{drawing_placed_layers, update_path_geometry, DrawingMutation};
use crate::schema::geometry::editing::{translate_world_path_points, PathPointRef};
use crate::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DragPathPoints, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    let layers = super::mutation::drag_path_points_layers(&payload.targets);
    let duplicated = payload.targets.iter().enumerate().any(|(index, target)| payload.targets.iter().take(index).any(|previous| previous == target));
    if !(payload.dx.is_finite() && payload.dy.is_finite()) || payload.targets.is_empty() || duplicated {
        return Ok(Vec::new());
    }
    let ids: Vec<_> = layers.iter().collect();
    let placed = drawing_placed_layers(base, &ids);
    let mut restore = Vec::new();
    for id in &layers {
        let Some(entry) = placed.iter().find(|entry| &crate::schema::layer_base(entry.layer).id == id) else { continue };
        let DrawingLayerNode::Path(path) = entry.layer else { continue };
        if !entry.editable {
            continue;
        }
        let points: Vec<PathPointRef> = payload.targets.iter().filter(|target| &target.layer_id == id).map(|target| PathPointRef { index: target.index, point: target.point }).collect();
        let matrix = crate::schema::geometry::multiply(entry.parent, crate::schema::drawing_transform_to_matrix(&path.base.transform));
        let Ok(segments) = translate_world_path_points(&path.segments, &points, matrix, [payload.dx, payload.dy]) else { continue };
        if !segments.iter().eq(path.segments.iter()) {
            restore.push(update_path_geometry(path.base.id.clone(), path.segments.clone()));
        }
    }
    Ok(restore)
}
//#endregion 🔖️Inverse
