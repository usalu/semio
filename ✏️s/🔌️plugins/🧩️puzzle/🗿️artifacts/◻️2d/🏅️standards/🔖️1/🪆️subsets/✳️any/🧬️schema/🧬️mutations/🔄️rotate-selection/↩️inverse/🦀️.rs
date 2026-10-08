//! ↩️ Inverse for `RotateSelection` — the absolute setters restoring every BASE position and handle the
//! rotation turns (exact, never a negated angle that would accumulate float error). Nothing moved ⇒
//! `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{move_node, puzzle2d_rotated, puzzle2d_selection, replace_node_handle, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RotateSelection, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle2d_selection(base, &payload.targets, false) else { return Ok(Vec::new()) };
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) {
        return Ok(Vec::new());
    }
    let (pivot, radians) = ((payload.pivot_x, payload.pivot_y), payload.angle);
    let (sin, cos) = radians.sin_cos();
    let mut steps = Vec::new();
    for node in &selection.nodes {
        if puzzle2d_rotated((node.x, node.y), pivot, sin, cos) != (node.x, node.y) {
            steps.push(move_node(node.id.clone(), node.x, node.y));
        }
        steps.extend(node.handles.iter().filter(|handle| handle.angle + radians != handle.angle).map(|handle| replace_node_handle(node.id.clone(), handle.id.clone(), handle.clone())));
    }
    Ok(steps)
}
//#endregion 🔖️Inverse
