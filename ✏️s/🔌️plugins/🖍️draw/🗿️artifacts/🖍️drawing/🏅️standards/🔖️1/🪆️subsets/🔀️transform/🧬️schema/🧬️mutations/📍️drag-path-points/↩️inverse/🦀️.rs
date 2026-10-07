//! ↩️ Inverse for `DragPathPoints` — the absolute geometry of every path the drag reshapes, captured from BASE.
use crate::mutations::{update_path_geometry, DrawingMutation};
use crate::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DragPathPoints, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    super::diff::diff(payload, base)
        .diff()
        .layers
        .iter()
        .flat_map(|delta| delta.patched.iter())
        .filter_map(|entry| match crate::schema::find_drawing_layer(base, &entry.id) {
            Some(DrawingLayerNode::Path(path)) => Some(update_path_geometry(entry.id.clone().into(), path.segments.clone())),
            _ => None,
        })
        .collect()

    })())
}
//#endregion 🔖️Inverse
