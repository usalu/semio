//! ↩️ Inverse for `ScaleSelection` — the absolute setters restoring every BASE position and extent the
//! scaling changes (exact, never a reciprocal factor that would accumulate float error). Nothing moved ⇒
//! `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{move_node, move_target_region, puzzle2d_selection, resize_target_region, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ScaleSelection, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    let Ok(selection) = puzzle2d_selection(base, &payload.targets, true) else { return Ok(Vec::new()) };
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.factor.is_finite() && payload.factor > 0.0) {
        return Ok(Vec::new());
    }
    let (cx, cy, factor) = (payload.pivot_x, payload.pivot_y, payload.factor);
    let mut steps = Vec::new();
    for node in &selection.nodes {
        if cx + (node.x - cx) * factor != node.x || cy + (node.y - cy) * factor != node.y {
            steps.push(move_node(node.id.clone(), node.x, node.y));
        }
    }
    for region in &selection.regions {
        if cx + (region.x - cx) * factor != region.x || cy + (region.y - cy) * factor != region.y {
            steps.push(move_target_region(region.id.clone(), region.x, region.y));
        }
        if region.width * factor != region.width || region.height * factor != region.height {
            steps.push(resize_target_region(region.id.clone(), region.width, region.height));
        }
    }
    Ok(steps)
}
//#endregion 🔖️Inverse
