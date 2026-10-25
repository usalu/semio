//! 📐️ rotation turns (exact, never a negated angle that would accumulate float error). Nothing moved ⇒
//! ↩️ Inverse for `RotateSelection` — the absolute setters restoring every BASE position and handle the
//! `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::move_node::MoveNode;
use crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::ReplaceNodeHandle;
use {crate::standards::v1::subsets::any::schema::mutations::move_node, crate::standards::v1::subsets::any::schema::mutations::selection::puzzle2d_rotated, crate::standards::v1::subsets::any::schema::mutations::selection::puzzle2d_selection, crate::standards::v1::subsets::any::schema::mutations::replace_node_handle};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::component::RotateSelection;

//#region 🔖️Inverse
pub fn inverse<Op: From<RotateSelection> + From<MoveNode> + From<ReplaceNodeHandle>>(payload: &super::RotateSelection, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle2d_selection(base, &payload.targets, false) else { return Ok(Vec::new()) };
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) {
        return Ok(Vec::new());
    }
    let (pivot, radians) = ((payload.pivot_x, payload.pivot_y), payload.angle);
    let (sin, cos) = radians.sin_cos();
    let mut steps = Vec::new();
    for node in &selection.nodes {
        if puzzle2d_rotated((node.x, node.y), pivot, sin, cos) != (node.x, node.y) {
            steps.push(move_node::move_node::<Op>(node.id.clone(), node.x, node.y));
        }
        steps.extend(node.handles.iter().filter(|handle| handle.angle + radians != handle.angle).map(|handle| replace_node_handle::replace_node_handle::<Op>(node.id.clone(), handle.id.clone(), handle.clone())));
    }
    Ok(steps)
}
//#endregion 🔖️Inverse
