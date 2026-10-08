//! ↩️ Inverse for `DragSelection` — the absolute setters restoring every BASE position the drag moves
//! (exact, never a negated offset that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{move_node, move_target_region, puzzle2d_selection, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragSelection, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    let (dx, dy) = (payload.dx, payload.dy);
    let Ok(selection) = puzzle2d_selection(base, &payload.targets, true) else { return Ok(Vec::new()) };
    if !(dx.is_finite() && dy.is_finite()) {
        return Ok(Vec::new());
    }
    let nodes = selection.nodes.iter().filter(|node| node.x + dx != node.x || node.y + dy != node.y).map(|node| move_node(node.id.clone(), node.x, node.y));
    let regions = selection.regions.iter().filter(|region| region.x + dx != region.x || region.y + dy != region.y).map(|region| move_target_region(region.id.clone(), region.x, region.y));
    Ok(nodes.chain(regions).collect())
}
//#endregion 🔖️Inverse
